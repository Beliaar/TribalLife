extends Resource
class_name TerrainDefinition

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var valid_voxels : Array
var invalid_voxels : Array
var spatial : Spatial
var center : Vector3
var location : Vector3
var building: Building
var outline_mesh: Mesh

# Called when the node enters the scene tree for the first time.
func _ready():	
	pass # Replace with function body.

func update_mesh(): 
	pass

func check_terrain_and_update_blocks(var start: Vector3, var v_tool: VoxelTool) -> bool:
	return true
# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
