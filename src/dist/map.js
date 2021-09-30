var map = L.map('map').setView([63.42682, 10.40163], 13);

L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
	maxZoom: 18,
	attribution: 'Map data © <a href="https://openstreetmap.org">OpenStreetMap</a> contributors',
	id: 'osm.standard'
}).addTo(map);

/* https://github.com/Boscop/web-view/issues/289#issuecomment-878961859 */
function sendMessageToServer(cmd) {
    if (window.external !== undefined) {
        return window.external.invoke(cmd);
    } else if (window.webkit.messageHandlers.external !== undefined) {
        return window.webkit.messageHandlers.external.postMessage(cmd);
    }
    throw new Error('Failed to locate webkit external handler')
}

var maker = L.marker();
map.on('click', function(event) {
    maker.remove()
    maker = L.marker(event.latlng);
    maker.addTo(map);
    sendMessageToServer([event.latlng.lat, event.latlng.lng]);
});
