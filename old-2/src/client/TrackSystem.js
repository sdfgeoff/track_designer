const DB = require('../db.js');

TrackSystem = function() {}

TrackSystem.init = function() {
	// Data
	DB.create_table("wheel_specification", {"id": "number", "x": 'number', "y": 'number'})
	DB.create_table("idler_specification", {"id": "number", 'radius':'number'})
	DB.create_table("drive_wheel_specification", {"id": "number", 'segments':'number'})
	DB.create_table("track_specification", {"id": "number", "track_pitch": 'number', 'thickness': 'number', 'flexible':'boolean'})
	DB.create_table("track_system", {"id": "number", "number_segments": "number"})
	
	// Relations
	DB.create_table("idlers", {"wheel_specification": 'number',  "idler_specification":'number', "track_system": 'number', "position":"number"})
	DB.create_table("drive_wheels", {"wheel_specification": 'number',  "drive_wheel_specification": 'number', "track_system": 'number', "position":"number"})
	DB.create_table("tracks", {"track_system": 'number', "track_specification": 'number'})
}


TrackSystem.create_track_system = function() {
	// Shortcut for creating a specification + system + tracks
	// returns the track system ID
}


TrackSystem.append_idler = function(system, position, radius) {
	// Creates a wheel at the end of a track system
}

TrackSystem.append_drive_wheel = function(system, position, segment_repeats) {
	// Creates a wheel at the end of a track system
}

TrackSystem.get_wheel_array = function(system) {
	// Returns an array of all the wheels in the specified system.
	// All wheels have the property "radius" - which is computed for drive wheels
}
