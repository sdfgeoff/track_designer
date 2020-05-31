TrackSystem = function(){}

TrackSystem.TABLES = {
	'Idlers': [],
	'DriveWheels': [],
	'TrackSpecifications': []
	'BaseWheels': []
	'TrackSystems': []
}


TrackSystem.create_idler = function(radius, base_wheel) {
	TrackSystem.TABLES['Idlers'].append({
		'radius': radius,
		'base_wheel': base_wheel
	})
	return TrackSystem.TABLES['Idlers'].length
}

TrackSystem.Idler.
