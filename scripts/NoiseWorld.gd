extends Spatial

var terrain : VoxelTerrain
var stream = VoxelGenerator_DF.new()
#var stream = preload("res://VoxelGenerator_DF.cs").new()

const MATERIAL = preload("res://materials/grass-rock.material")
const ELEVATION = preload("res://region2-00250-01-01-el.png")
const BIOME = preload("res://region2-00250-01-01-bm.png")

#const Generator = preload("res://VoxelGenerator_DF.cs").new()
var water_types = ["ocean", "lake"]
var area_radius = 2


func _ready():
	terrain = $VoxelTerrain
	terrain.stream = stream
	stream.max_height = 300
	stream.elevation_map = ELEVATION
	stream.biome_map = BIOME	
	var file = File.new()
	
	file.open("res://biome_color_key.txt", File.READ)
	var i = 1
	while !file.eof_reached():
		var line = file.get_line()
		if line.empty():
			continue
		var data = BiomeData.new()
		var split = line.rsplit(" ", false, 1)
		var name_def : PoolStringArray = split[0].rsplit(" ", false, 1)
		if name_def.size() > 1:
			data.name = name_def[1]
			data.modifiers = name_def[0].split(" ")
		else:
			data.name = name_def[0]			
		var color_array = split[1].lstrip('()').rstrip(')').split(',')
		while color_array.size() < 3:
			color_array.push_back(0)
		var r = int(color_array[0])
		var g = int(color_array[1])
		var b = int(color_array[2])
		var color = Color8(r, g, b)
		data.voxel_type = i
		stream.biome_dictionary[color] = data
		terrain.voxel_library.voxel_count += 1
		var voxel = terrain.voxel_library.create_voxel(i, data.name)
		voxel.material_id = 0
		if water_types.has(data.name):
			voxel.color = Color.sandybrown
		else:
			voxel.color = color
		voxel.geometry_type = Voxel.GEOMETRY_CUBE
		i += 1
	terrain.voxel_library.bake()
	stream.update_map_data()
	var ground_def = CubeTerrainDefinition.new()
	ground_def.location = Vector3(0,0,0)
	ground_def.invalid_voxels.push_back(0)
	ground_def.size.x = 3
	ground_def.size.z = 3
	ground_def.size.y = 1
	ground_def.center.x = 1
	ground_def.center.z = 1
	$Building.terrain_definitions.push_back(ground_def)

	var floor_def = CubeTerrainDefinition.new()
	floor_def.location = Vector3(0,1,0)
	floor_def.valid_voxels.push_back(0)
	floor_def.size.x = 3
	floor_def.size.z = 3
	floor_def.size.y = 1
	floor_def.center.x = 1
	floor_def.center.z = 1
	$Building.terrain_definitions.push_back(floor_def)

	var bulk_def = CubeTerrainDefinition.new()
	bulk_def.location = Vector3(0,2,0)
	bulk_def.valid_voxels.push_back(0)
	bulk_def.size.x = 3
	bulk_def.size.z = 2
	bulk_def.size.y = 2
	bulk_def.center.x = 1
	bulk_def.center.z = 1
	$Building.terrain_definitions.push_back(bulk_def)
	$Building.create_childs()
	pass

func _physics_process(delta):
	$TextureRect.update()
	$TextureRect2.update()
	var camera : Camera = $VoxelTerrain.get_node($VoxelTerrain.viewer_path)
	var space_state = get_world().direct_space_state
	var from = camera.global_transform.origin	
	var to = from - camera.global_transform.basis.z * 10
	var hit : Dictionary = space_state.intersect_ray(from,to)
	if hit:		
		var vtool : VoxelTool = $VoxelTerrain.get_voxel_tool()
		var point_pos = hit["position"].round()
		point_pos.x += 0.5
		point_pos.z += 0.5
		point_pos.y -= 0.5
		$Building.translation = point_pos
		$Building.check_terrain(point_pos, vtool)
		return
						
func _input(event):
	if event is InputEventKey:
		if event.pressed && event.scancode == KEY_M:
			$TextureRect.visible = !$TextureRect.visible
	pass

func _on_TextureRect_draw():
	
	var camera = $VoxelTerrain.get_node($VoxelTerrain.viewer_path)
		
	var global_cam = $VoxelTerrain.to_global(camera.translation)
	var max_width = ELEVATION.get_size().x
	var max_height = ELEVATION.get_size().y
	var vec = Vector2(global_cam.x + max_width / 2, global_cam.z + max_height / 2)
	
	vec = vec / Vector2(max_width, max_height)
	vec = vec * $TextureRect.rect_size
	var rect_size = Vector2(10, 10)
	
	$TextureRect.draw_rect(Rect2(vec - rect_size / 2, rect_size), Color.white, false)



func _on_TextureRect2_draw():
	var camera = $VoxelTerrain.get_node($VoxelTerrain.viewer_path)
		
	var global_cam = $VoxelTerrain.to_global(camera.translation)
	var max_width = ELEVATION.get_size().x
	var max_height = ELEVATION.get_size().y
	var vec = Vector2(global_cam.x + max_width / 2, global_cam.z + max_height / 2)
	
	vec = vec / Vector2(max_width, max_height)
	vec = vec * $TextureRect2.rect_size
	var rect_size = Vector2(10, 10)
	
	$TextureRect2.draw_rect(Rect2(vec - rect_size / 2, rect_size), Color.white, false)
	
