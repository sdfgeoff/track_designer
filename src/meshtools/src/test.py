mesh_1 = {
    'vert_data': [
        (0.0, 0.0, 0.0),
        (0.1, 0.0, 0.0),
        (0.2, 0.0, 0.0),
        (0.3, 0.0, 0.0),
        (0.4, 0.0, 0.0),
        (0.5, 0.0, 0.0),
        (0.6, 0.0, 0.0),
    ],
    'face_indices': [
    ],
    'vertex_groups': {
        'edge_left': [
            0,1,2,3,4,5,6
        ]
    }
}

mesh_2 = {
    'vert_data': [
        (0.0, 0.0, 1.0),
        (0.1, 0.0, 1.0),
        (0.2, 0.0, 1.0),
        (0.3, 0.0, 1.0),
        (0.4, 0.0, 1.0),
        (0.5, 0.0, 1.0),
        (0.6, 0.0, 1.0),
    ],
    'face_indices': [
    ],
    'vertex_groups':{
        'edge_left': [
            0,1,2,3,4,5,6
        ]
    }
}

def dist_sq(p1, p2):
    d = [
    p1[0] - p2[0],
    p1[1] - p2[1],
    p1[2] - p2[2]
    ]
    return d[0] ** 2 + d[1] ** 2 + d[2] ** 2

def join_edges(vert_data, edge_1_indices, edge_2_indices):
    p1 = 1
    p2 = 1

    faces = []

    while True:
        vert_1 = vert_data[edge_1_indices[p1]]
        vert_2 = vert_data[edge_2_indices[p2]]
        vert_2_next = vert_data[edge_2_indices[p2 + 1]]

        dist_here = dist_sq(vert_1, vert_2)
        dist_next = dist_sq(vert_1, vert_2_next)

        if dist_here < dist_next:
            faces.append((
                edge_1_indices[p1-1],
                edge_1_indices[p1],
                edge_2_indices[p2-1]
            ))

            p1 += 1
            if p1 >= len(edge_1_indices) - 1:
                while p2 < len(edge_2_indices) - 1:
                    faces.append((
                        edge_1_indices[p1-1],
                        edge_2_indices[p2],
                        edge_2_indices[p2-1]
                    ))
                    p2 += 1
                break
        else:
            faces.append((
                edge_1_indices[p1-1],
                edge_2_indices[p2],
                edge_2_indices[p2-1]
            ))
            p2 += 1
            if p2 >= len(edge_2_indices) - 1:
                while(p1 < len(edge_1_indices) ):
                    faces.append((
                        edge_1_indices[p1 - 1],
                        edge_1_indices[p1],
                        edge_2_indices[p2-1]
                    ))
                    p1 += 1
                break

    print(faces)
    return faces

def join_meshes(mesh_1, mesh_2, edge_group_names):
    combined_verts = mesh_1['vert_data'] + mesh_2['vert_data']
    mesh_2_vert_offset = len(mesh_1['vert_data'])
    face_indices = []
    for fid in mesh_1['face_indices']:
        face_indices.append(fid)
    for fid in mesh_2['face_indices']:
        face_indices.append(
            (i+mesh_2_vert_offset for i in fid)
        )

    for group_name in edge_group_names:
        edge_1_indices = mesh_1['vertex_groups'][group_name]
        edge_2_indices = [e + mesh_2_vert_offset for e in mesh_2['vertex_groups'][group_name]]

    face_indices += join_edges(combined_verts, edge_1_indices, edge_2_indices)
    # TODO vertex group merging

    return {
        'vert_data': combined_verts,
        'face_indices': face_indices
    }


def draw_mesh(mesh):
    tris = []

    for face in mesh['face_indices']:
        pos = []
        for p in face:
            pos.append(mesh['vert_data'][p])
            tris.append(pos)
    #exit()

    from mpl_toolkits.mplot3d import Axes3D
    from mpl_toolkits.mplot3d.art3d import Poly3DCollection
    import matplotlib.pyplot as plt
    fig = plt.figure()
    ax = Axes3D(fig)
    for tri in tris:
        p = Poly3DCollection(tri)
        p.set_edgecolor('r')
        ax.add_collection3d(p)
    plt.show()

combined_mesh = join_meshes(mesh_1, mesh_2, ['edge_left'])
draw_mesh(combined_mesh)
