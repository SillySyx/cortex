const version = '0.2';
const ignoreFiles = [
    "serviceworker.js",
];

async function cache_request(request) {
    const cached = await caches.match(request);
    if (cached) {
        return cached;
    }

    const response = await fetch(request);

    const cache = await caches.open(`data-${version}`);
    cache.put(request, response.clone());

    return response;
}

self.addEventListener("fetch", event => {
    if (self.location.hostname === "localhost" || self.location.hostname === "127.0.0.1")
        return;
        
    if (ignoreFiles.some(file => event.request.url.endsWith(file)))
        return;

    if (event.request.method !== "GET")
        return;

    return event.respondWith(cache_request(event.request));
});