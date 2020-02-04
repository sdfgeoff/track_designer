function polar_to_cartesian(center, radius, angle) {
	return [
		center[0] + Math.cos(angle) * radius,
		center[1] - Math.sin(angle) * radius, // SVG has zero at the top
	]
}

function normalize_vector(vec) {
	let length = get_length(vec)
	
	return [
		vec[0] / length,
		vec[1] / length,
	]
}

function add_vec(vec1, vec2) {
	return [
		vec1[0] + vec2[0],
		vec1[1] + vec2[1],
	]
}

function subtract_vec(vec1, vec2){
	return [
		vec1[0] - vec2[0],
		vec1[1] - vec2[1],
	]
}

function scale_vec(vec1, scale){
	return [
		vec1[0] * scale,
		vec1[1] * scale,
	]
}

function get_length(vec) {
	return Math.sqrt(vec[0]*vec[0] + vec[1]*vec[1])
}


function rotate_90(vec){
	return [
		vec[1], 
		-vec[0]
	]
}
