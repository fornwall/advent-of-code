#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Copy, Clone)]
struct Triangle {
    points: [Point3D; 3],
}

pub type ObjectId = u32;
pub type ShapeId = u32;
pub type KeyFrame = u32;

#[derive(Debug)]
enum Shape {
    /// Single triangle facing upwards. Corners:
    /// TopLeft:     { x: -0.5, y:  0.5 }
    /// TopRight:    { x:  0.5, y:  0.5 }
    /// BottomLeft:  { x: -0.5, y: -0.5 }
    /// BottomRight: { x:  0.5, y: -0.5 }
    Cube,
    /// Circle inside cube (see Cube above).
    Circle,
    LineSegments {
        width: f32,
        points: Vec<Point3D>,
    },
    /// Single triangle facing upwards. Corners:
    /// Top:         { x:  0.0, y:  0.5 }
    /// BottomLeft:  { x: -0.5, y: -0.5 }
    /// BottomRight: { x:  0.5, y: -0.5 }
    Triangle,
    Triangles {
        triangles: Vec<Triangle>,
    },
    Multiple {
        shapes: Box<Shape>,
    },
}

#[derive(Debug)]
pub enum VisualizationEvent {
    SpawnObject {
        object_id: ObjectId,
        shape_id: ShapeId,
    },
    MoveObject {
        object_id: ObjectId,
        to: Point3D,
    },
    Recolor {
        object_id: ObjectId,
        color: Color,
    },
    DeleteObject {
        object_id: ObjectId,
    },
    OrthographicCamera {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        // near & far computed?
    },
}

#[derive(Debug)]
pub struct VisualizationEventWithTime {
    pub key_frame: KeyFrame,
    pub event: VisualizationEvent,
}

#[derive(Debug, Default)]
pub struct Visualization {
    pub objects: Vec<Shape>,
    pub shapes: Vec<Shape>,
    pub events: Vec<VisualizationEventWithTime>,
}

impl Visualization {
    pub fn num_keyframes(&self) -> u32 {
        self.events[self.events.len() - 1].key_frame
    }

    pub fn add_shape(&mut self, shape: Shape) -> ShapeId {
        let shape_id = self.shapes.len() as ShapeId;
        self.shapes.push(shape);
        shape_id
    }

    pub fn add_object(&mut self, key_frame: KeyFrame, shape_id: ShapeId) -> ObjectId {
        let object_id = self.objects.len() as ObjectId;
        self.events.push(VisualizationEventWithTime {
            key_frame,
            event: VisualizationEvent::SpawnObject {
                object_id,
                shape_id,
            },
        });
        object_id
    }

    pub fn translate_object(&mut self, key_frame: KeyFrame, object_id: ObjectId, to: Point3D) {
        self.events.push(VisualizationEventWithTime {
            key_frame,
            event: VisualizationEvent::MoveObject { object_id, to },
        });
    }

    pub fn recolor_object(&mut self, key_frame: KeyFrame, object_id: ObjectId, color: Color) {
        self.events.push(VisualizationEventWithTime {
            key_frame,
            event: VisualizationEvent::Recolor { object_id, color },
        });
    }

    pub fn add_object_with_position_and_color(
        &mut self,
        key_frame: KeyFrame,
        shape_id: ShapeId,
        position: Point3D,
        color: Color,
    ) -> ObjectId {
        let object_id = self.add_object(key_frame, shape_id);
        self.translate_object(key_frame, object_id, position);
        self.recolor_object(key_frame, object_id, color);
        object_id
    }

    pub fn add_event(&mut self, event: VisualizationEventWithTime) {
        self.events.push(event);
    }
}
