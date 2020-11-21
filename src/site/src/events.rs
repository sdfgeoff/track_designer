enum Events {
    AnimationFrame(f32);
    
    //~ WheelAdded,
    //~ WheelRemoved,
    //~ WheelChanged(u16),
    
    //~ WheelOrderChanged,
    //~ TrackPathSegmentChanged(u16),
    
    //~ TrackInterfaceChanged,
    //~ TrackSurfaceChanged,
    
    //~ /// Parameters dialog should show the parameters for this entity
    //~ ObjectSelected(SelectableObjects),
    
    //~ WheelMeshChanged(u16),
    //~ TrackMeshChanged,
    
    //~ SchematicMouseDown,
    //~ SchematicMouseUp,
    //~ SchematicMouseDrag,
    
    //~ VisualizerMouseDown,
    //~ VisualizerMouseUp,
    //~ VisualizerMouseDrag,
    
    
    //~ DatabaseLoad(Vec<u8>),
    //~ DatabaseSaveRequest,
    //~ DownloadFileRequest(????), // How to handle what file?
}


enum SelectableObjects {
    None, // Sowns downlaod file buttons
    Track, // Allows configuration of segment length, selecting a tread, track width etc.
    Wheel(u16), // Allows configuration of position, radius (for idlers)
}
