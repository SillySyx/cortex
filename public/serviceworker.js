const version = '0.3';
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

function should_use_caching(request) {
    if (self.location.hostname === "localhost" || self.location.hostname === "127.0.0.1")
        return false;

    if (ignoreFiles.some(file => request.url.endsWith(file)))
        return false;

    if (request.method !== "GET")
        return false;

    return true;
}

async function remove_old_caches() {
    const keys = await caches.keys();

    for (const key of keys) {
        if (key !== `data-${version}`) {
            caches.delete(key);
        }
    }
}

self.addEventListener("fetch", event => {
    if (should_use_caching(event.request)) {
        event.respondWith(cache_request(event.request));
    }
});

self.addEventListener('activate', event => {
    event.waitUntil(remove_old_caches());
});
  