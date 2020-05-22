extends VoxelGenerator
class_name VoxelGenerator_DF

export var elevation_map : Texture 
export var biome_map : Texture 
export var max_height = 100 
export var biome_dictionary : Dictionary

var map_height_data = []
var map_type_data = []
var size: Vector2
var data_calculated = false

# Called when the node enters the scene tree for the first time.
func _ready():
	if !data_calculated:
		update_map_data()
	pass # Replace with function body.

func update_map_data():
	if elevation_map == null:
		return
	var image = elevation_map.get_data()
	image.lock()
	var biomeImage = null
	if biome_map != null:
		biomeImage = biome_map.get_data()
		biomeImage.lock()

	size = image.get_size()
	map_height_data.clear()
	map_type_data.clear()
	map_height_data.resize(size.x * size.y)
	map_type_data.resize(size.x * size.y)

	for x in range(size.x):
		for z in range(size.y):
			var map_pos = Vector2(x, z)
			var type = 1;
			if biomeImage != null:
				var biomeColor = biomeImage.get_pixelv(map_pos)
				if biome_dictionary.has(biomeColor):
					var biome_data : BiomeData
					biome_data = biome_dictionary[biomeColor]
					type = biome_data.voxel_type
			var color = image.get_pixelv(map_pos)
			var value = color.v
			var height = max_height * value
			var index = z * size.y + x
			map_height_data[index] = height
			map_type_data[index] = type
	
	image.unlock();
	if biomeImage != null:
		biomeImage.unlock()
	data_calculated = true
	

func get_used_channels_mask() -> int:
	return 1 << VoxelBuffer.CHANNEL_TYPE	

func generate_block(buffer: VoxelBuffer, origin: Vector3, lod: int) -> void:
	for x in range(buffer.get_size_x()):
		for z in range(buffer.get_size_z()):
			var map_pos = Vector2(
			size.x / 2.0 + origin.x + x,
			size.y / 2.0 + origin.z + z)
			if (!(map_pos.x < size.x) || !(map_pos.y < size.y)
				|| !(map_pos.x > 0) || (!map_pos.y > 0)):
				continue
			var index = map_pos.y * size.y + map_pos.x
			var height = map_height_data[index]
			var type =  map_type_data[index] 
			var topY = origin.y + buffer.get_size_y()
			if topY < height:
				for y in range(buffer.get_size_y()):
					buffer.set_voxel(type, x, y, z)
			elif origin.y < max_height && height > origin.y:
				for y in range(topY - height):
					buffer.set_voxel(type, x, y, z)

	pass
# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
