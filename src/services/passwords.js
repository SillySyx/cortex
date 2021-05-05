import { v4 as uuidv4 } from 'uuid';

import { createEncryptedDataEntry, parseEncryptedDataEntry, encrypt, decrypt, loadKey } from './crypto';

export class PasswordService {
    async loadCategory(id) {
		let [loaded, passwords] = await loadPasswords();
        if (!loaded)
            return [false, null];

        for (const category of passwords) {
            if (category.id === id) {
                return [true, category];
            }
        }

        return [false, null];
    }

    async createCategory(title) {
		let [loaded, passwords] = await loadPasswords();
        if (!loaded)
            return false;

        passwords.push({
            id: uuidv4(),
            timestamp: Date.now(),
            title: title,
            passwords: [],
        });

        return await storePasswords(passwords);
    }

    async saveCategory(id, title) {
		let [loaded, passwords] = await loadPasswords();
        if (!loaded)
            return false;

        let category = passwords.find(item => item.id === id);
        if (!category)
            return false;
            
        category.title = title;
        category.timestamp = Date.now();

        return await storePasswords(passwords);
    }

    async removeCategory(id) {
		let [loaded, passwords] = await loadPasswords();
        if (!loaded)
            return false;

        passwords = passwords.filter(category => category.id !== id);

        return await storePasswords(passwords);
    }

    async listPasswords() {
		let [loaded, passwords] = await loadPasswords();
        if (!loaded)
            return [false, null];

        passwords.sort((a, b) => a.title.toLowerCase().localeCompare(b.title.toLowerCase()));
        for (const category of passwords) {
            category.passwords.sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()));
        }

        return [true, passwords];
    }

    async loadPassword(id) {
		let [loaded, passwords] = await loadPasswords();
        if (!loaded)
            return [false, null];

        for (const category of passwords) {
            for (const password of category.passwords) {
                if (password.id === id) {
                    return [true, password];
                }
            }
        }

        return [false, null];
    }

    async createPassword(categoryId, name, description, password) {
		let [loaded, passwords] = await loadPasswords();
        if (!loaded)
            return false;

        let category = passwords.find(category => category.id === categoryId);
        if (!category)
            return false;

        category.passwords.push({
            id: uuidv4(),
            timestamp: Date.now(),
            name: name,
            description: description,
            password: password,
        });

        return await storePasswords(passwords);
    }

    async savePassword(passwordId, name, description, password) {
		let [loaded, passwords] = await loadPasswords();
        if (!loaded)
            return false;

        let category = passwords.find(category => category.passwords.some(password => password.id === passwordId));
        if (!category)
            return false;

        let _password = category.passwords.find(password => password.id === passwordId);
        if (!_password)
            return false;

        _password.name = name;
        _password.description = description;
        _password.password = password;
        _password.timestamp = Date.now();
        
        return await storePasswords(passwords);
    }

    async removePassword(passwordId) {
		let [loaded, passwords] = await loadPasswords();
        if (!loaded)
            return false;

        let category = passwords.find(category => category.passwords.some(password => password.id === passwordId));
        if (!category)
            return false;

        category.passwords = category.passwords.filter(password => password.id !== passwordId)

        return await storePasswords(passwords);
    }

    async mergePasswords(newPasswords) {
		let [loaded, passwords] = await loadPasswords();
        if (!loaded)
            return;

        for (const newCategory of newPasswords) {
            const category = passwords.find(c => c.id === newCategory.id);
            if (!category) {
                passwords.push(newCategory);
                continue;
            }

            if (newCategory.timestamp > category.timestamp) {
                category.title = newCategory.title;
                category.timestamp = newCategory.timestamp;
            }

            for (const newPassword of newCategory.passwords) {
                const password = category.passwords.find(p => p.id === newPassword.id);
                if (!password) {
                    category.passwords.push(newPassword);
                    continue;
                }

                if (newPassword.timestamp > password.timestamp) {
                    password.name = newPassword.name;
                    password.description = newPassword.description;
                    password.password = newPassword.password;
                    password.timestamp = newPassword.timestamp;
                }
            }
        }

        storePasswords(passwords);
    }
}

async function loadPasswords() {
    const [keyLoaded, key] = loadKey();
    if (!keyLoaded)
        return [false, null];

    const encryptedDataEntry = localStorage.getItem("passwords");
    if (!encryptedDataEntry)
        return [true, []];

    const [iv, data] = parseEncryptedDataEntry(encryptedDataEntry);

    const [decrypted, decryptedData] = await decrypt(key, iv, data);
    if (!decrypted)
        return [false, null];

    const passwords = JSON.parse(decryptedData);

    return [true, passwords];
}

async function storePasswords(passwords) {
    const [keyLoaded, key] = loadKey();
    if (!keyLoaded)
        return false;

    const passwordData = JSON.stringify(passwords);

    const [encrypted, iv, data] = await encrypt(key, passwordData);
    if (!encrypted)
        return false;

    const encryptedDataEntry = createEncryptedDataEntry(iv, data);
    localStorage.setItem("passwords", encryptedDataEntry);

    return true;
}