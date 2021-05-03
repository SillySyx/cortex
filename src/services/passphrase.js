import { adjectives, nouns, verbs } from './wordlist.json';

export function generatePassphrase() {
    return `${randomWord(adjectives)} ${randomWord(nouns)} ${randomWord(verbs)} ${randomWord(adjectives)} ${randomWord(nouns)}`;
}

function randomWord(wordlist) {
    const randomIndex = Math.floor(Math.random() * wordlist.length);
    return wordlist[randomIndex];
}