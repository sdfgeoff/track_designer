"use strict"

const Diagram = function () {
	self = this
}

Diagram.clear = function(svg) {
	while (svg.lastChild) {
		svg.removeChild(svg.lastChild)
	}
	
	let background = document.createElementNS("http://www.w3.org/2000/svg", "rect")
	background.setAttribute("width", "100%")
	background.setAttribute("height", "100%")
	background.setAttribute("fill", "white")
	svg.appendChild(background)
}


Diagram.draw_idler_wheel = function(svg, location, track_center_radius, belt_thickness) {
	let idler_outline = document.createElementNS("http://www.w3.org/2000/svg", "circle")
	idler_outline.setAttribute("cx", num_to_cm(location[0]))
	idler_outline.setAttribute("cy", num_to_cm(location[1]))
	idler_outline.setAttribute("r", num_to_cm(track_center_radius - (belt_thickness / 2)))
	idler_outline.setAttribute("fill", "none")
	idler_outline.setAttribute("stroke", "black")
	idler_outline.setAttribute("stroke-width", "0.2mm")

	svg.appendChild(idler_outline)
}


Diagram.draw_track_straight_section = function(svg, start_location, stop_location, segment_length, belt_thickness) {
	let centerline = create_line(start_location, stop_location)
	centerline.setAttribute("fill", "none")
	centerline.setAttribute("stroke", "black")
	centerline.setAttribute("stroke-dasharray", "0.5mm, 0.5mm")
	centerline.setAttribute("stroke-width", "0.2mm")
	
	
	let direction = normalize_vector(subtract_vec(stop_location, start_location))
	let track_normal = rotate_90(direction)
	let thickness_vec = scale_vec(track_normal, belt_thickness/2)
	
	let track_outer = create_line(add_vec(start_location, thickness_vec), add_vec(stop_location, thickness_vec))
	track_outer.setAttribute("fill", "none")
	track_outer.setAttribute("stroke", "black")
	track_outer.setAttribute("stroke-width", "0.2mm")
	
	let track_inner = create_line(subtract_vec(start_location, thickness_vec), subtract_vec(stop_location, thickness_vec))
	track_inner.setAttribute("fill", "none")
	track_inner.setAttribute("stroke", "black")
	track_inner.setAttribute("stroke-width", "0.2mm")
	
	svg.appendChild(centerline)
	svg.appendChild(track_outer)
	svg.appendChild(track_inner)
}

Diagram.draw_track_arc = function(svg, center, radius, start_angle, wrap_angle, belt_thickness) {
	let end_angle = start_angle + wrap_angle

	let centerline = create_arc(center, radius, start_angle, end_angle)
	centerline.setAttribute("fill", "none")
	centerline.setAttribute("stroke", "black")
	centerline.setAttribute("stroke-dasharray", "0.5mm, 0.5mm")
	centerline.setAttribute("stroke-width", "0.2mm")
	
	let track_outside = create_arc(center, radius + belt_thickness / 2, start_angle, end_angle)
	track_outside.setAttribute("fill", "none")
	track_outside.setAttribute("stroke", "black")
	track_outside.setAttribute("stroke-width", "0.2mm")
	
	let track_inside = create_arc(center, radius - belt_thickness / 2, start_angle, end_angle)
	track_inside.setAttribute("fill", "none")
	track_inside.setAttribute("stroke", "black")
	track_inside.setAttribute("stroke-width", "0.2mm")

	svg.appendChild(centerline)
	svg.appendChild(track_outside)
	svg.appendChild(track_inside)
}


Diagram.draw_dimension_line = function(svg, start_position, end_position, offset) {
	let line_vec = subtract_vec(end_position, start_position)
	let line_direction = normalize_vector(line_vec)
	let line_tangent = rotate_90(line_direction)
	
	let offset_vec = scale_vec(line_tangent, offset)
	let offset_vec2 = scale_vec(line_tangent, offset + 2)
	
	let length = get_length(line_vec)
	
	
	let lines = [
		create_line(start_position, add_vec(start_position, offset_vec2)),
		create_line(end_position, add_vec(end_position, offset_vec2)),
		create_line(add_vec(start_position, offset_vec), add_vec(end_position, offset_vec)),
	]
	
	let mid_point = add_vec(add_vec(start_position, offset_vec2), scale_vec(line_vec, 0.5))
	
	for (let line of lines) {
		line.setAttribute("fill", "none")
		line.setAttribute("stroke", "black")
		line.setAttribute("stroke-width", "0.2mm")
		svg.appendChild(line)
	}
	
	let dimension = create_text(length.toFixed(2), mid_point)
	dimension.setAttribute('fill', 'black');
	dimension.setAttribute('font-size', '3mm');
	svg.appendChild(dimension)
	
	let arrowhead1 = Diagram.create_arrowhead(add_vec(start_position, offset_vec), scale_vec(line_direction, -1))
	let arrowhead2 = Diagram.create_arrowhead(add_vec(end_position, offset_vec), line_direction)
	svg.appendChild(arrowhead1)
	svg.appendChild(arrowhead2)
	
}



Diagram.create_arrowhead = function(tip_position, direction) {
	const length = 3
	const width = 1
	
	
	let direction_norm = normalize_vector(direction)
	let tail_position = subtract_vec(tip_position, scale_vec(direction_norm, length))
	
	let tangent = rotate_90(direction_norm)
	let tail_offset = scale_vec(tangent, width)
	
	let points = [
		tip_position,
		add_vec(tail_position, tail_offset),
		subtract_vec(tail_position, tail_offset)
	]
	
	let arrowhead = create_polygon(points)
	return arrowhead
}

