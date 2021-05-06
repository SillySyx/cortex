const cacheName = "0.6";

self.addEventListener("install", event => {
    self.skipWaiting();
});

self.addEventListener("fetch", event => {
    if (event.request.method !== "GET")
        return;

    if (self.location.hostname === "localhost" || self.location.hostname === "127.0.0.1")
        return;

    event.respondWith((async () => {
        let response = await caches.match(event.request);
        if (response)
            return response;

        response = await fetch(event.request);
        const cache = await caches.open(cacheName);
        cache.put(event.request, response.clone());
        return response;
    })());
});

self.addEventListener("activate", event => {
    event.waitUntil(caches.keys().then(keys => {
        Promise.all(keys.map(key => {
            if (key === cacheName) 
                return;

            caches.delete(key);
        }))
    }));
});