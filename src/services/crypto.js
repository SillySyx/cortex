export async function generateKeyFromSeed(seed) {
    if (!crypto.subtle)
        return [false, null];

    try {
        const encoded = new TextEncoder().encode(seed);
        const hashed = await crypto.subtle.digest("SHA-256", encoded);

        return [true, new Uint8Array(hashed)];
    }
    catch {
        return [false, null];
    }
}

export async function encrypt(key, data) {
    if (!crypto.subtle)
        return [false, null, null];

    try {
        const iv = crypto.getRandomValues(new Uint8Array(12));

        const algorithm = {
            name: "AES-GCM",
            iv: iv,
        };

        const cryptoKey = await crypto.subtle.importKey("raw", key, algorithm, false, ["encrypt"]);

        const encodedData = new TextEncoder().encode(data);

        const encryptedData = await crypto.subtle.encrypt(algorithm, cryptoKey, encodedData);

        return [true, iv, new Uint8Array(encryptedData)];
    }
    catch {
        return [false, null, null];
    }
}

export async function decrypt(key, iv, encryptedData) {
    if (!crypto.subtle)
        return [false, null];

    try {
        const algorithm = {
            name: "AES-GCM",
            iv: iv,
        };

        const cryptoKey = await crypto.subtle.importKey("raw", key, algorithm, false, ["decrypt"]);

        const decryptedData = await crypto.subtle.decrypt(algorithm, cryptoKey, encryptedData);

        const decodedData = new TextDecoder().decode(decryptedData);

        return [true, decodedData];
    }
    catch {
        return [false, null];
    }
}

export function loadKey() {
    const key = sessionStorage.getItem("key");
    if (!key) {
        return [false, null];
    }

    const parsedKey = JSON.parse(key);

    return [true, new Uint8Array(parsedKey)];
}

export function storeKey(key) {
    if (!key) {
        return false;
    }

    const parsedKey = JSON.stringify([...key]);

    sessionStorage.setItem("key", parsedKey);

    return true;
}

export async function verifyKey(key) {
    const verification = localStorage.getItem("verification");
    if (!verification) {
        const [encrypted, iv, data] = await encrypt(key, "valid");
        if (!encrypted) {
            return false;
        }

        const entry = createEncryptedDataEntry(iv, data);
        localStorage.setItem("verification", entry);

        return true;
    }

    const [iv, bytes] = parseEncryptedDataEntry(verification);
    const [decrypted, data] = await decrypt(key, iv, bytes);
    if (!decrypted) {
        return false;
    }

    return data === "valid";
}

export function createEncryptedDataEntry(iv, data) {
    return JSON.stringify({
        iv: [...iv],
        bytes: [...data]
    });
}

export function parseEncryptedDataEntry(entry) {
    const { iv, bytes } = JSON.parse(entry);
    return [new Uint8Array(iv), new Uint8Array(bytes)];
}