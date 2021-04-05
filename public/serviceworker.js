const version = '0.1';

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
    if (event.request.method !== "GET")
        return;

    if (event.request.url.indexOf(":8080") > -1)
        return;

    return event.respondWith(cache_request(event.request));
});