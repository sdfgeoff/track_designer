function create_line(start_location, end_location) {
	let line = document.createElementNS("http://www.w3.org/2000/svg", "line")
	
	line.setAttribute("x1", num_to_cm(start_location[0]))
	line.setAttribute("y1", num_to_cm(start_location[1]))
	line.setAttribute("x2", num_to_cm(end_location[0]))
	line.setAttribute("y2", num_to_cm(end_location[1]))
	
	return line
}



function create_arc(center, radius, start_angle, end_angle){
	let arc = document.createElementNS("http://www.w3.org/2000/svg", "path")
	arc.setAttribute("d", _compute_arc(center, radius, start_angle, end_angle))
	return arc
}

function _compute_arc(center, radius, start_angle, end_angle) {
	let start_pos = polar_to_cartesian(center, radius, start_angle)
	let end_pos = polar_to_cartesian(center, radius, end_angle)

	let large_arc = (end_angle - start_angle) <= Math.PI ? "0" : "1"
	let arc = "M " + [num_to_cm(start_pos[0]), num_to_cm(start_pos[1])].join(" ")
	arc += " A " + [num_to_cm(radius), num_to_cm(radius), 0, large_arc, 0, num_to_cm(end_pos[0]), num_to_cm(end_pos[1])].join(" ")
	return arc
}


function create_text(text, position) {
	let text_elem = document.createElementNS("http://www.w3.org/2000/svg", "text")
	text_elem.setAttribute('x', num_to_cm(position[0]))
	text_elem.setAttribute('y', num_to_cm(position[1]))
	text_elem.textContent = text
	return text_elem
}

function num_to_cm(num) {
	// 1cm = 35.43 pixels
	// num is in mm
	return num * 35.43307 * 0.1
}


function create_polygon( points) {
	let poly = document.createElementNS("http://www.w3.org/2000/svg", "polygon")
	let flat_array = []
	
	let cm_points = points.map(x => num_to_cm(x[0]) + "," + num_to_cm(x[1]))
	poly.setAttribute('points', cm_points)
	return poly
}
