var map = L.map('map').setView([63.42682, 10.40163], 13);

L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
	maxZoom: 18,
	attribution: 'Map data © <a href="https://openstreetmap.org">OpenStreetMap</a> contributors',
	id: 'osm.standard'
}).addTo(map);

var maker = L.marker();
map.on('click', function(event) {
    maker.remove()
    maker = L.marker(event.latlng);
    maker.addTo(map);
    external.invoke([event.latlng.lat, event.latlng.lng]);
});
