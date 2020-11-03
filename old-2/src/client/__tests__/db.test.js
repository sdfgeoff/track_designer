const DB = require('../db.js');

test('create_table', () => {
	DB.clear()
	expect(DB.create_table("test_table", {'col':'number'})).toBe(true)
	expect(DB.get_schema("test_table")).toStrictEqual({'col':'number'})
})

test('cant_create_duplicate_table', () => {
	DB.clear()
	expect(DB.create_table("test_table", {})).toBe(true)
	expect(DB.create_table("test_table", {})).toBe(false)
})

test('get_nonexistant_schema', () => {
	DB.clear()
	expect(DB.get_schema("test_table", {})).toBe(undefined)
})

test('insert_row', () => {
	DB.clear()
	expect(DB.create_table("test_table", {'num':'number'})).toBe(true)
	expect(DB.insert("test_table", {'num': 5})).toBe(true)
})

test('insert_to_nonexistant_table', () => {
	DB.clear()
	expect(DB.insert("nonexistant_table", {'thing':4})).toBe(false)
})

test('insert_with_incorrect_object', () => {
	DB.clear()
	expect(DB.create_table("test_table", {'num':'number'})).toBe(true)
	expect(DB.insert("test_table", {'num': "asdf"})).toBe(false)
})

test('validate_object', () => {
	let schema = {'num':'number', "string":'string', "bool":'boolean'}
	expect(DB.validate_object(schema, {"num":3, "string":"asdf", "bool":true})).toBe(true)
	expect(DB.validate_object(schema, {"num":"fail", "string":"asdf", "bool":true})).toBe(false)
	expect(DB.validate_object(schema, {"extras":false})).toBe(false)
})

test('validate_schema', () => {
	expect(DB.validate_schema({'num':'number', "string":'string', "bool":'boolean'})).toBe(true)
	expect(DB.validate_schema({'num':'asdf'})).toBe(false)
})

test('get_all', () => {
	DB.clear()
	expect(DB.create_table("test_table", {'num':'number'})).toBe(true)
	expect(DB.insert("test_table", {'num': 5})).toBe(true)
	expect(DB.insert("test_table", {'num': 5})).toBe(true)
	expect(DB.insert("test_table", {'num': 7})).toBe(true)
	expect(DB.get_all("test_table")).toStrictEqual([
		{'num': 5},
		{'num': 5},
		{'num': 7}
	])
})

test('get_all_by_value', () => {
	DB.clear()
	expect(DB.create_table("test_table", {'num':'number', "val":'boolean'})).toBe(true)
	expect(DB.insert("test_table", {'num': 5, 'val':true})).toBe(true)
	expect(DB.insert("test_table", {'num': 5, 'val':false})).toBe(true)
	expect(DB.insert("test_table", {'num': 7, 'val':true})).toBe(true)
	expect(DB.get_all_by_value("test_table", "num", 5)).toStrictEqual([
		{'num': 5, 'val':true},
		{'num': 5, 'val':false}
	])
	
	expect(DB.get_all_by_value("test_table", "num", 7)).toStrictEqual([
		{'num': 7, 'val':true},
	])
})


