import bpy
import bmesh


TEMPLATE = """
use meshtools::mesh::{{Mesh, Vertex, Face, VertWeight}};

pub fn get_mesh() -> Mesh {{
    Mesh {{
        vertices: vec![
{verts}
        ],
        faces: vec![
{faces}
        ],
        vertex_groups: vec![
{groups}
        ].into_iter().collect()
    }}  
}}
"""

def export_mesh(obj):
    mesh = obj.data
    
    # Triangulate mesh
    tri_mesh = bmesh.new()
    tri_mesh.from_mesh(mesh)
    bmesh.ops.triangulate(
        tri_mesh, faces=tri_mesh.faces, quad_method="ALTERNATE")
    
    
    vertices = []
    faces = []
    vertex_groups = {}
    
    layer_deform = tri_mesh.verts.layers.deform.active
    if layer_deform is None:
        raise Exception("No Vertex Groups")
    vertex_group_names = names = tuple(vertex_group.name for vertex_group in obj.vertex_groups)
    
    
    for vert in tri_mesh.verts:
        vertices.append((vert.co.x, vert.co.y, vert.co.z))
        
        for vertex_group_index, weight in vert[layer_deform].items():
            if vertex_group_index not in vertex_groups:
                vertex_groups[vertex_group_index] = []
            
            vertex_groups[vertex_group_index].append((vert.index, weight))
            
        
    for face in tri_mesh.faces:
        faces.append((
            face.verts[0].index,
            face.verts[1].index,
            face.verts[2].index,
        ))
    
    
    group_strings = []
    for group in vertex_groups:
        vert_weights = vertex_groups[group]
        
        # This is a hack to get the bridge vertices to work nicely. It
        # works because the vertiies are now in some sort of order.
        # The alternative would be to be to implement a more intelligent
        # generate_vertex_bridge function.
        #vert_weights.sort(key=lambda v: vertices[v[0]][1])
        
        group_strings.append('''
            (
                "{name}".to_string(), 
                vec![
{vert_weights}
                ]
            ),'''.format(
                name=vertex_group_names[group],
                vert_weights = ",\n".join(["                    VertWeight::new({}, {})".format(*w) for w in vert_weights])
            )
        )
        
        
        
    

    open("{}.rs".format(mesh.name.lower()), "w").write(TEMPLATE.format(
        verts=",\n".join(["            Vertex::new{}".format(v) for v in vertices]),
        faces=",\n".join(["            Face{}".format(f) for f in faces]),
        groups="\n".join(group_strings)
        
    ))

for obj in bpy.data.objects:
    export_mesh(obj)
