export async function writeToClipboard(value) {
    navigator.clipboard.writeText(value);
}