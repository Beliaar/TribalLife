extends TerrainDefinition
class_name CubeTerrainDefinition

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var cubes : Dictionary
var size : Vector3 = Vector3(1,1,1)

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func update_mesh():
	var spatial = Spatial.new()
	var min_x = location.x
	var max_x = location.x + size.x
	
	var min_y = location.y
	var max_y = location.y + size.y

	var min_z = location.z
	var max_z = location.z + size.z
	for x in range(min_x, max_x):
		for y in range(min_y, max_y):
			for z in range(min_z, max_z):
				var instance = MeshInstance.new()
				var cube = CubeMesh.new()
				cube.size = Vector3(1.5,1.5,1.5)
				var adjusted_x = x - center.x
				var adjusted_y = y - center.y
				var adjusted_z = z - center.z
				instance.translation.x = -0.2 + adjusted_x + (x * 0.2)
				instance.translation.y = +0.2 + adjusted_y - (y * 0.2)
				instance.translation.z = -0.2 + adjusted_z + (z * 0.2)
				cube.material = SpatialMaterial.new()
				cube.material.albedo_color = Color.white
				instance.mesh = cube
				var cube_id = "%d,%d,%d" % [adjusted_x, adjusted_y, adjusted_z]
				cubes[cube_id] = cube
				spatial.add_child(instance)
				if center == Vector3(adjusted_x, adjusted_y, adjusted_z):
					var center_cube = cube.duplicate()
					center_cube.material = SpatialMaterial.new()
					center_cube.material.albedo_color = Color.white
					center_cube.material.albedo_color.a = 0.5
					center_cube.material.flags_transparent = true
					var center_instance = MeshInstance.new()
					center_instance.mesh = center_cube
					spatial.add_child(center_instance)
	#spatial.translation = location
	self.spatial = spatial

func check_terrain_and_update_blocks(var start: Vector3, var v_tool: VoxelTool) -> bool:
	var start_position = start
	var valid = true
	var min_x = location.x
	var max_x = location.x + size.x
	
	var min_y = location.y
	var max_y = location.y + size.y

	var min_z = location.z
	var max_z = location.z + size.z
	for x in range(min_x, max_x):
		for y in range(min_y, max_y):
			for z in range(min_z, max_z):
				var adjusted_x = x - center.x
				var adjusted_y = y - center.y
				var adjusted_z = z - center.z
				var position = start_position + Vector3(adjusted_x, adjusted_y, adjusted_z)
				var cube_id = "%d,%d,%d" % [adjusted_x, adjusted_y, adjusted_z]
				var cube = cubes[cube_id]
				var v_id = v_tool.get_voxel(position)
				var block_valid = true
				if !self.valid_voxels.empty():
					block_valid = self.valid_voxels.has(v_id)
				if !self.invalid_voxels.empty():
					block_valid = !self.invalid_voxels.has(v_id)
				if !block_valid:
					cube.material.albedo_color = Color.red
				else:
					cube.material.albedo_color = Color.green	
					cube.material.albedo_color.a = 0.1
					cube.material.flags_transparent = true
				valid = valid && block_valid
	return valid

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
