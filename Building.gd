extends Spatial
class_name Building

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var terrain_definitions : Array
var can_be_placed : bool = false
var center : Vector3 = Vector3(0,0,0)

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func create_childs():
	for child in self.get_children():
		self.remove_child(child)
	for def in terrain_definitions:
		def.update_mesh()
		self.add_child(def.spatial)

func check_terrain(var start: Vector3, var v_tool: VoxelTool):
	var is_valid = true
	for def in terrain_definitions:
		if !def.check_terrain_and_update_blocks(start, v_tool):
			is_valid = false
	can_be_placed = is_valid

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
