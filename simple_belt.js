"use strict"
/*
This calculator aims to simplify the creation of tracks for small
remote controlled vehicles.

This is the "simple" case where there is an infinitely flexible belt
wrapped around two equal size wheels


Typical Use Cases:
 - Buy an existing belt (defines spacing, length, thickness) and wheels, calculate inter-wheel-distance
 - Create a custom belt provided inter-wheel-distance and wheel radius


The system is fully constrained by the following parameters:
*/
function SystemParameters() {
	self = this
	self.track_pitch = 1.0
	self.track_segment_repeats = 1.0
	self.wheel_segment_repeats = 1.0
	self.belt_thickness = 1.0
}

/* 
From these parameters, everything we are interested in can be computed
*/
function compute_wheel_circumference(parameters) {
	return parameters.track_pitch * parameters.wheel_segment_repeats
}

function compute_wheel_radius(parameters) {
	return compute_wheel_circumference(parameters) / 2 / Math.PI
}

function compute_axle_height(parameters) {
	return compute_wheel_radius(parameters) + parameters.belt_thickness / 2
}

function compute_track_length(parameters) {
	return parameters.track_pitch * parameters.track_segment_repeats
}

function compute_inter_wheel_distance(parameters) {
	let track_length = compute_track_length(parameters)
	let wheel_circumference = compute_wheel_circumference(parameters)
	return (track_length - wheel_circumference) / 2
}

/*
There are the following constraint/modification functions.

Each function modifies the parameters based on it's constraint and a "priority".
A priority of "1" means the constraint should enforce it's parameter immediately.
A priority of "0" means the constraint should have no effect.
*/

function define_track_segment_repeats(parameters, priority, repeats) {
	let current_repeats = parameters.track_segment_repeats
	let error = repeats - current_repeats
	error *= priority
	
	parameters.track_segment_repeats += error
	return error
}

function define_wheel_segment_repeats(parameters, priority, repeats) {
	let current_repeats = parameters.wheel_segment_repeats
	let error = repeats - current_repeats
	error *= priority
	parameters.wheel_segment_repeats += error
	return error
}


function define_track_repeats_integer(parameters, priority, _no_input) {
	let current_repeats = parameters.track_segment_repeats
	let error = Math.round(current_repeats) - current_repeats
	error *= priority
	
	parameters.track_segment_repeats += error
	return error
}

function define_wheel_repeats_integer(parameters, priority, _no_input) {
	let current_repeats = parameters.wheel_segment_repeats
	let error = Math.round(current_repeats) - current_repeats
	error *= priority
	
	parameters.wheel_segment_repeats += error
	return error
}

function define_segment_length(parameters, priority, length) {
	let current_length = parameters.track_pitch
	let error = length - current_length
	error *= priority
	parameters.track_pitch += error
	return error
}

function define_belt_thickness(parameters, priority, length) {
	let current_length = parameters.belt_thickness
	let error = length - current_length
	error *= priority
	parameters.belt_thickness += error
	return error
}

function define_wheel_radius(parameters, priority, wheel_radius) {
	let current_wheel_radius = compute_wheel_radius(parameters)
	let error = wheel_radius - current_wheel_radius
	
	// Wheel radius is not an intrinsic property of the system.
	// This works by attempting to reduce the number of repeats
	// of the track around the wheel and the length of each track section.
	
	error *= priority
	
	parameters.wheel_segment_repeats += error / parameters.track_pitch * 2 * Math.PI * (1/2)
	parameters.track_pitch += error / parameters.wheel_segment_repeats * (1/2)
	
	return error
}

function define_wheel_separation(parameters, priority, separation) {
	let current_separation = compute_inter_wheel_distance(parameters)
	let error = separation - current_separation
	
	// Wheel separation is not an intrinsic property of the system.
	// This works by attempting to change:
	//  - the length of the track (both segment length and repeats)
	//  - adjust the wheel radius to free up more track
	
	error *= priority
	
	parameters.track_segment_repeats += error / parameters.track_pitch * (1/3)
	parameters.track_pitch += error / parameters.track_segment_repeats * (1/3)
	parameters.wheel_segment_repeats -= error / parameters.wheel_segment_repeats * 2 * Math.PI * (1/3)
	
	
	return error
}


function system_sanity(parameters, _no_priority, _no_input){
	parameters.track_segment_repeats = Math.max(1, parameters.track_segment_repeats)
	parameters.wheel_segment_repeats = Math.max(1, parameters.wheel_segment_repeats)
	parameters.track_pitch = Math.max(0.1, parameters.track_pitch)
	return 0
}


const CONSTRAINT_FUNCTIONS = {
	"system_sanity": system_sanity,
	"define_track_repeats_integer": define_track_repeats_integer,
	"define_wheel_repeats_integer": define_wheel_repeats_integer,
	"define_segment_length": define_segment_length,
	"define_wheel_radius": define_wheel_radius,
	"define_wheel_separation": define_wheel_separation,
	"define_wheel_segment_repeats": define_wheel_segment_repeats,
	"define_track_segment_repeats": define_track_segment_repeats,
	"define_belt_thickness": define_belt_thickness,
}


const CONSTANT_CONTRAINTS = [
	{"id": "define_track_repeats_integer", "priority": 1.0},
	{"id": "define_wheel_repeats_integer", "priority": 1.0},
	{"id": "system_sanity", "priority": 1.0}
]


/* To solve it, we run through each of the constraints iteratively */
function iterate(system_parameters, input_parameters, constraints, priority_modifier) {
	let error = 0
	
	for (let constraint of constraints){
		let constraint_function = CONSTRAINT_FUNCTIONS[constraint["id"]]
		let input = input_parameters[constraint["input"]]
		let priority = constraint["priority"]
		
		error += Math.abs(constraint_function(
			system_parameters,
			priority * priority_modifier,
			input,
		))
	}
	
	
	return error
}



const TRACK_PITCH = {"id": "track_pitch", 'default_value': 7.5, 'unit': 'mm', 'min':0, 'max':50, 'name': 'Track Pitch'}
const TRACK_SEGMENT_REPEATS = {"id": "track_segment_repeats", 'default_value': 20, 'unit': 'lugs', 'min':0, 'max':100, 'name': 'Track Segment Count'}
const TRACK_BELT_THICKNESS = {"id": "track_belt_thickness", 'default_value': 1, 'unit': 'mm', 'min':0, 'max':10, 'name': 'Belt Thickness'}
const WHEEL_SEGMENT_REPEATS = {"id": "wheel_segment_repeats", 'default_value': 8, 'unit': 'lugs', 'min':0, 'max':20, 'name': 'Wheel Segment Count'}
const APPROX_WHEEL_SPACING = {"id": "approx_wheel_spacing", 'default_value': 33, 'unit': 'mm', 'min':0, 'max':100, 'name': 'Approximate Wheel Spacing'}
const APPROX_WHEEL_RADIUS = {"id": "approx_wheel_radius", 'default_value': 20, 'unit': 'mm', 'min':0, 'max':100, 'name': 'Approximate Wheel Radius'}
const APPROX_TRACK_PITCH = {"id": "approx_track_pitch", 'default_value': 15, 'unit': 'mm', 'min':0, 'max':50, 'name': 'Approximate Track Pitch'}


const SOLVER_MODES = {
	"existing_belt_find_spacing": {
		"inputs": [
			TRACK_PITCH,
			TRACK_SEGMENT_REPEATS,
			TRACK_BELT_THICKNESS,
			WHEEL_SEGMENT_REPEATS,
		],
		"constraints": [
			{"id": "define_segment_length", "priority": 1.0, "input": "track_pitch"},
			{"id": "define_track_segment_repeats", "priority": 1.0, "input": "track_segment_repeats"},
			{"id": "define_belt_thickness", "priority": 1.0, "input": "track_belt_thickness"},
			{"id": "define_wheel_segment_repeats", "priority": 1.0, "input": "wheel_segment_repeats"},
		]
	},
	"existing_belt_fit_dimensions": {
		"inputs": [
			TRACK_PITCH,
			TRACK_SEGMENT_REPEATS,
			TRACK_BELT_THICKNESS,
			APPROX_WHEEL_SPACING,
		],
		"constraints": [
			{"id": "define_wheel_separation", "priority": 1.0, "input": "approx_wheel_spacing"},
			{"id": "define_segment_length", "priority": 1.0, "input": "track_pitch"},
			{"id": "define_belt_thickness", "priority": 1.0, "input": "track_belt_thickness"},
			{"id": "define_track_segment_repeats", "priority": 1.0, "input": "track_segment_repeats"},
		]
		
	},
	"fit_dimensions": {
		"inputs": [
			TRACK_BELT_THICKNESS,
			APPROX_TRACK_PITCH,
			APPROX_WHEEL_SPACING,
			APPROX_WHEEL_RADIUS,
		],
		"constraints": [
			{"id": "define_segment_length", "priority": 0.3, "input": "approx_track_pitch"},
			{"id": "define_belt_thickness", "priority": 1.0, "input": "track_belt_thickness"},
			{"id": "define_wheel_separation", "priority": 2.5, "input": "approx_wheel_spacing"},
			{"id": "define_wheel_radius", "priority": 2.0, "input": "approx_wheel_radius"},
		]
	},
}



let INPUT_PARAMETERS = {}
let CONSTRAINTS = []


function init() {
	console.log("init")
	
	let solver_mode_box = document.getElementById("set_solver")
	solver_mode_box.onchange = function() {
		set_solver_mode(solver_mode_box.value)
	}
	set_solver_mode(solver_mode_box.value)
	
}


function set_solver_mode(mode) {
	// Regenerate Input Widgets
	let input_box = document.getElementById("inputs")
	input_box.innerHTML = ""
	
	INPUT_PARAMETERS = {}
	CONSTRAINTS = SOLVER_MODES[mode]['constraints']

	for (let param of SOLVER_MODES[mode]['inputs']) {
		// Separating this variable out is needed so that it can be
		// captured by the closure that adjusts input parameters
		let param_id = param.id 
		
		
		INPUT_PARAMETERS[param_id] = param.default_value
		
		let container = document.createElement("div")
		container.className = "parameter_container"
		
		let name = document.createElement("div")
		name.innerHTML = param.name
		name.className = "name"
		
		let range = document.createElement("input")
		range.setAttribute("type", "range")
		range.value = param.default_value
		range.min = param.min
		range.max = param.max
		range.className = "slider"
		
		let number = document.createElement("input")
		number.setAttribute("type", "number")
		number.value = param.default_value
		number.step = 0.5
		number.className = "value"
		
		let unit = document.createElement("div")
		unit.innerHTML = param.unit
		unit.className = "unit"
		
		container.appendChild(name)
		container.appendChild(range)
		container.appendChild(number)
		container.appendChild(unit)
		
		input_box.appendChild(container)
		
		function update_parameter_raw(new_value){
			number.value = new_value
			range.value = new_value
			
			INPUT_PARAMETERS[param_id] = parseFloat(new_value)
			
			if (document.getElementById("auto_run").checked) {
				run()
			}
		}
		
		function update_parameter_value(event) {
			update_parameter_raw(event.target.value)
			
		}
		
	
		number.onchange = update_parameter_value
		range.oninput = update_parameter_value
		
	}
	
	
	run()
}



// What happens when the user wants the output to update
function run() {
	let system_parameters = new SystemParameters()
	let input_parameters = INPUT_PARAMETERS
	let constraints = CONSTRAINTS
	
	// Do some initial approximation
	//~ parameters.track_pitch = INPUT_PARAMETERS.belt_segment_length
	//~ let wheel_circumference = INPUT_PARAMETERS.wheel_radius.value * 2 * Math.PI
	//~ let track_length = wheel_circumference + INPUT_PARAMETERS.wheel_separation.value * 2
	//~ parameters.wheel_segment_repeats = wheel_circumference / INPUT_PARAMETERS.segment_length.value
	//~ parameters.track_segment_repeats = track_length / INPUT_PARAMETERS.segment_length.value
	
	
	console.log("computing")
	let prev_error = 0;
	let error = 0;
	let iterations = 0;
	
	let stop_reason = ""
	let failed = false
	
	
	while (true) {
		let percent_completed = iterations / 1000
		prev_error = error
		
		error = iterate(system_parameters, input_parameters, constraints, (1.0 - percent_completed))
		error += iterate(system_parameters, input_parameters, CONSTANT_CONTRAINTS, 1.0)
		
		iterations += 1
		if (iterations > 1000) {
			stop_reason = "Iteration > 1000"
			failed = true
			break
		}
		if (Math.abs(prev_error - error) < 0.01) {
			stop_reason = "\u0394Error < 0.01"
			break
		}
		if (error < 0.01) {
			stop_reason = "Error < 0.01"
			break
		}
	}

	console.log("drawing")
	let svg = document.getElementById("diagram")
	draw(system_parameters, svg)
	
	let svg_link = document.getElementById("download_svg")
	let svg_source = svg.outerHTML
	var file = new Blob(Array.from(svg_source), {type: 'image/svg'})
	var fileURL = URL.createObjectURL(file)
	svg_link.setAttribute("href", fileURL)
	
	generate_outputs(system_parameters, iterations, error, stop_reason, failed)
}


function generate_outputs(parameters, iterations, error, stop_reason, failed) {
	let outputs = document.getElementById("outputs")
	outputs.innerHTML = ""
	
	function add_output(output_name, output_value, output_unit) {
		let container = document.createElement("div")
		container.className = "parameter_container"
		if (failed) {
			container.className += " failed"
		}
		
		let name = document.createElement("div")
		name.innerHTML = output_name
		name.className = "name"
		
		let value = document.createElement("div")
		value.innerHTML = output_value
		value.className = "output_value"
		
		let unit = document.createElement("div")
		unit.innerHTML = output_unit
		unit.className = "unit"
		
		container.appendChild(name)
		container.appendChild(value)
		container.appendChild(unit)
		
		outputs.appendChild(container)
	}
	
	
	add_output("Track Segments", parameters.track_segment_repeats.toFixed(2), "lugs")
	add_output("Wheel Segments", parameters.wheel_segment_repeats.toFixed(2), "lugs")
	add_output("Segment Length", parameters.track_pitch.toFixed(2), "mm")
	
	add_output("Track Length", compute_track_length(parameters).toFixed(2), "mm")
	add_output("Axle Height", compute_axle_height(parameters).toFixed(2), "mm")
	add_output("Wheel Separation", compute_inter_wheel_distance(parameters).toFixed(2), "mm")
	add_output("Belt Thickness", parameters.belt_thickness, "mm")
	
	add_output("-----", "--", "--")
	add_output("Solver Error", error.toFixed(2), "mm")
	add_output("Solver Iterations", iterations, stop_reason)
	
	
}

// Drawing the diagram
function draw(parameters, svg) {
	
	// Clear previous
	Diagram.clear(svg)
	
	let border_space = 20
	let wheel_radius = compute_wheel_radius(parameters)
	let inter_wheel_spacing = compute_inter_wheel_distance(parameters)
	
	let wheel_offset_x = wheel_radius + border_space + (parameters.belt_thickness / 2)
	let wheel_offset_y = wheel_radius + border_space + (parameters.belt_thickness / 2)
	
	let left_wheel_position = [wheel_offset_x, wheel_offset_y]
	let right_wheel_position = add_vec(left_wheel_position, [inter_wheel_spacing, 0])
	
	Diagram.draw_idler_wheel(svg, left_wheel_position, wheel_radius, parameters.belt_thickness)
	Diagram.draw_idler_wheel(svg, right_wheel_position, wheel_radius, parameters.belt_thickness)
	
	let top_segment_position = wheel_offset_y - wheel_radius
	let bottom_segment_position = wheel_offset_y + wheel_radius
	let left_segment_pos = wheel_offset_x
	let right_segment_pos = wheel_offset_x + inter_wheel_spacing
	
	Diagram.draw_track_arc(svg, left_wheel_position, wheel_radius, Math.PI/2, Math.PI, parameters.belt_thickness)
	Diagram.draw_track_arc(svg, right_wheel_position, wheel_radius, -Math.PI/2, Math.PI, parameters.belt_thickness)
	
	Diagram.draw_track_straight_section(svg, [left_segment_pos, top_segment_position], [right_segment_pos, top_segment_position], parameters.track_pitch, parameters.belt_thickness)
	Diagram.draw_track_straight_section(svg, [right_segment_pos, bottom_segment_position], [left_segment_pos, bottom_segment_position], parameters.track_pitch, parameters.belt_thickness)

	// Dimension inter-wheel space
	Diagram.draw_dimension_line(svg, left_wheel_position, right_wheel_position, wheel_radius + 10)
	
	// Dimension wheel directly
	Diagram.draw_dimension_line(svg, [left_segment_pos, bottom_segment_position - parameters.belt_thickness / 2], [left_segment_pos, top_segment_position + parameters.belt_thickness / 2], (wheel_radius + parameters.belt_thickness/2 + 5))
	
	// Dimension wheel + track
	Diagram.draw_dimension_line(svg, [left_segment_pos, bottom_segment_position + parameters.belt_thickness / 2], [left_segment_pos, top_segment_position - parameters.belt_thickness / 2], (wheel_radius + parameters.belt_thickness/2 + 15))
	
	// Dimension axle height
	Diagram.draw_dimension_line(svg, right_wheel_position, [right_segment_pos, bottom_segment_position + parameters.belt_thickness / 2], (wheel_radius + parameters.belt_thickness/2 + 5))
	
	/* Compute image size */
	let width = wheel_radius * 2 + border_space * 2 + inter_wheel_spacing + parameters.belt_thickness
	let height = 2 * wheel_radius + 2 * border_space + + parameters.belt_thickness
	svg.setAttribute("viewBox", [0,0, num_to_cm(width), num_to_cm(height)].join(" "))
}
