// db.js is a micro database that supports create, insert and update
// operations.


let DB = function(){}
DB.TABLES = {}
DB.SCHEMA = {}


DB.clear = function(){
	// Erases all entries from the 
	DB.TABLES = {}
	DB.SCHEMA = {}
}

DB.create_table = function(table_name, schema){
	if (table_name in DB.TABLES) {
		console.error("table_already_exists", {"name": table_name})
		return false;
	}
	DB.SCHEMA[table_name] = schema
	DB.TABLES[table_name] = []
	
	return true
}

DB.get_schema = function(table_name) {
	return DB.SCHEMA[table_name]
}

DB.insert = function(table_name, object) {
	let schema = DB.get_schema(table_name)
	if (schema == undefined) {
		console.error("table_does_not_exist", {"name": table_name})
		return false
	}
	if (!DB.validate_object(schema, object)) {
		console.error("object_does_not_match_schema")
		return false
	}
	DB.TABLES[table_name].push(object)
	return true
}

DB.delete = function(table_name, record) {
	if (!(table_name in DB.TABLES)) {
		console.error("table_does not exist", {"name": table_name})
		return false;
	}
	let index = DB.TABLES[table_name].indexOf(record)
	if (index == -1) {
		console.error("record_does_not_exist", {"record": record})
		return false
	}
	DB.TABLES[table_name].splice(index, 1)
}

DB.get_all_by_value = function(table_name, column, value) {
	return DB.TABLES[table_name].filter((x) => x[column] == value)
}

DB.get_all = function(table_name) {
	return DB.TABLES[table_name].map((x) => x) 
}

DB.validate_object = function(schema, object) {
	for (element in object){
		if (!(element in schema)) {
			console.error("contains_extra_element", {"element": element})
			return false
		}
		if (typeof(object[element]) !== schema[element]){
			console.error("incorrect_type", {"type": typeof(object[element]), "expected": schema[element]})
			return false
		}
	}
	return true
}

DB.validate_schema = function(schema) {
	for (element in schema){
		if (['number', 'string', 'boolean'].indexOf(schema[element]) == -1) {
			console.error("schema_unexpected_row_type", {"got": schema[element]})
			return false
		}
	}
	return true
}


module.exports = DB
