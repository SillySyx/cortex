export async function generateKey() {
    if (!crypto.subtle)
        return [false, null];

    try {
        const algorithm = {
            name: "AES-GCM",
            length: 256,
        };
        const extractable = true;
        const keyUsages = ["encrypt", "decrypt"];

        const key = await crypto.subtle.generateKey(algorithm, extractable, keyUsages);

        return [true, key];
    }
    catch(error) {
        console.error(error);
        return [false, null];
    }
}

function generateIvFromSeed(seed) {
    return crypto.getRandomValues(new Uint8Array([1,2,3,4,5,6,7,8,9,10,11,12]));
}

function encodeData(data) {
    const encoder = new TextEncoder();
    return encoder.encode(data);
}

function decodeData(data) {
    const decoder = new TextDecoder();
    return decoder.decode(data);
}

export async function encrypt(key, data) {
    if (!crypto.subtle)
        return [false, null];

    try {
        const iv = generateIvFromSeed("silly goose");
        const algorithm = {
            name: "AES-GCM",
            iv: iv,
        };
        const encodedData = encodeData(data);
        const arrayBuffer = await crypto.subtle.encrypt(algorithm, key, encodedData);

        return [true, arrayBuffer];
    }
    catch {
        return [false, null];
    }
}

export async function decrypt(key, data) {
    if (!crypto.subtle)
        return [false, null];

    try {
        const iv = generateIvFromSeed("silly goose");
        const algorithm = {
            name: "AES-GCM",
            iv: iv,
        };
        const arrayBuffer = await crypto.subtle.decrypt(algorithm, key, data);
        const decodedData = decodeData(arrayBuffer);

        return [true, decodedData];
    }
    catch {
        return [false, null];
    }
}

export async function loadKey() {
    if (!crypto.subtle)
        return false;

    try {
        const key = sessionStorage.getItem("key");
        if (!key) {
            return [false, null];
        }

        const rawKey = JSON.parse(key);
        const cryptoKey = await crypto.subtle.importKey("jwk", rawKey, "AES-GCM", false, ["encrypt", "decrypt"]);

        return [true, cryptoKey];
    }
    catch {
        return [false, null];
    }    
}

export async function storeKey(key) {
    if (!crypto.subtle)
        return false;

    try {
        const keyBuffer = await crypto.subtle.exportKey("jwk", key);
        
        sessionStorage.setItem("key", JSON.stringify(keyBuffer));

        return true;
    }
    catch {
        return false;
    }
}