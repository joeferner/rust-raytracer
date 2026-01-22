use std::collections::HashMap;
use std::sync::LazyLock;

use crate::docs::{ModuleDocs, ModuleDocsArguments};

pub(crate) static BUILTIN_MODULE_DOCS: LazyLock<HashMap<&'static str, ModuleDocs>> = LazyLock::new(
    || {
        let mut map = HashMap::new();

        // TODO Constants - PI
        // TODO Test Functions - is_undef
        // TODO Test Functions - is_bool
        // TODO Test Functions - is_num
        // TODO Test Functions - is_string
        // TODO Test Functions - is_list
        // TODO Test Functions - is_function
        // TODO Other - echo
        // TODO Other - render
        // TODO Other - children
        // TODO Other - assert
        // TODO Other - assign
        // TODO functions - concat
        // TODO functions - lookup
        // TODO functions - str
        // TODO functions - chr
        // TODO functions - ord
        // TODO functions - search
        // TODO functions - version
        // TODO functions - version_num
        // TODO functions - parent_module
        // TODO Mathematical Functions - abs
        // TODO Mathematical Functions - sign
        // TODO Mathematical Functions - sin
        // TODO Mathematical Functions - cos
        // TODO Mathematical Functions - tan
        // TODO Mathematical Functions - acos
        // TODO Mathematical Functions - asin
        // TODO Mathematical Functions - atan
        // TODO Mathematical Functions - atan2
        // TODO Mathematical Functions - floor
        // TODO Mathematical Functions - round
        // TODO Mathematical Functions - ceil
        // TODO Mathematical Functions - ln
        // TODO Mathematical Functions - len
        // TODO Mathematical Functions - let
        // TODO Mathematical Functions - log
        // TODO Mathematical Functions - pow
        // TODO Mathematical Functions - sqrt
        // TODO Mathematical Functions - exp
        // TODO Mathematical Functions - rands
        // TODO Mathematical Functions - min
        // TODO Mathematical Functions - max
        // TODO Mathematical Functions - norm
        // TODO Mathematical Functions - cross
        // TODO 2D Primitives - import
        // TODO 3D Primitives - import
        // TODO 3D Primitives - surface

        // Caustic objects
        map.insert(
            "camera",
            ModuleDocs {
                description:
                    "Creates a camera for rendering the scene. All parameters must be named."
                        .to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "aspect_ratio".to_owned(),
                        description: "Width/height ratio of the output image.".to_owned(),
                        default: Some("1.0".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "image_width".to_owned(),
                        description: "Width of the rendered image in pixels.".to_owned(),
                        default: Some("100".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "image_height".to_owned(),
                        description: "Height of the rendered image in pixels.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "samples_per_pixel".to_owned(),
                        description: "Number of random samples per pixel for anti-aliasing."
                            .to_owned(),
                        default: Some("10".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "max_depth".to_owned(),
                        description: "Maximum number of ray bounces in the scene.".to_owned(),
                        default: Some("10".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "vertical_fov".to_owned(),
                        description: "Vertical field of view in degrees.".to_owned(),
                        default: Some("90".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "look_from".to_owned(),
                        description: "Camera position as [x, y, z].".to_owned(),
                        default: Some("[0, 0, 0]".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "look_at".to_owned(),
                        description: "Point the camera is looking at as [x, y, z].".to_owned(),
                        default: Some("[0, -1, 0]".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "up".to_owned(),
                        description: "The up vector for the camera [x, y, z].".to_owned(),
                        default: Some("[0, 0, 1]".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "defocus_angle".to_owned(),
                        description:
                            "Variation angle of rays through each pixel (0 for no defocus blur)."
                                .to_owned(),
                        default: Some("0".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "focus_distance".to_owned(),
                        description:
                            "Distance from camera look_from point to plane of perfect focus."
                                .to_owned(),
                        default: Some("10".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "background".to_owned(),
                        description: "Background color as [r, g, b] (values 0-1).".to_owned(),
                        default: Some("[0, 0, 0]".to_owned()),
                    },
                ],
                examples: vec![
                    "camera();".to_owned(),
                    "camera(aspect_ratio=16.0/9.0, image_width=1200);".to_owned(),
                    "camera(look_from=[0, 2, 5], look_at=[0, 0, 0], vertical_fov=60);".to_owned(),
                    "camera(samples_per_pixel=100, max_depth=50, defocus_angle=0.6);".to_owned(),
                    "camera(background=[0, 0, 0], look_from=[3, 3, 2], look_at=[0, 0, -1]);"
                        .to_owned(),
                ],
            },
        );

        map.insert(
            "lambertian",
            ModuleDocs {
                description: "Creates a Lambertian (diffuse) material with the specified texture."
                    .to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "t".to_owned(),
                        description: "texture for the Lambertian material. Can be a color value or texture object."
                            .to_owned(),
                        default: None,
                    },
                ],
                examples: vec![
                    "lambertian([0.5, 0.5, 0.5]);".to_owned(),
                    "lambertian(checker_texture);".to_owned(),
                ],
            },
        );

        map.insert(
            "metal",
            ModuleDocs {
                description: "Creates a metal material with configurable color and fuzziness for reflections."
                    .to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "c".to_owned(),
                        description: "metal color as RGB vector [r,g,b] with values 0-1, or single grayscale value."
                            .to_owned(),
                        default: Some("white".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "fuzz".to_owned(),
                        description: "fuzziness factor for reflections (0=perfect mirror, 1=maximum diffusion)."
                            .to_owned(),
                        default: Some("0.2".to_owned()),
                    },
                ],
                examples: vec![
                    "metal([0.8, 0.8, 0.8]);".to_owned(),
                    "metal([0.8, 0.6, 0.2], 0.3);".to_owned(),
                    "metal([0.8, 0.8, 0.8], fuzz=0.1);".to_owned(),
                    "metal(0.7);".to_owned(),
                ],
            },
        );

        map.insert(
            "dielectric",
            ModuleDocs {
                description:
                    "Creates a dielectric (glass-like) material with a given refractive index."
                        .to_owned(),
                arguments: vec![ModuleDocsArguments {
                    name: "n".to_owned(),
                    description: "refractive index of the dielectric material.".to_owned(),
                    default: None,
                }],
                examples: vec![
                    "dielectric(1.5);".to_owned(),
                    "dielectric(n=1.5);".to_owned(),
                ],
            },
        );

        map.insert(
            "checker",
            ModuleDocs {
                description:
                    "Creates a checker pattern texture for materials. All parameters must be named."
                        .to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "scale".to_owned(),
                        description: "size of the checker pattern squares.".to_owned(),
                        default: Some("1".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "even".to_owned(),
                        description: "color or texture for even squares.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "odd".to_owned(),
                        description: "color or texture for odd squares.".to_owned(),
                        default: None,
                    },
                ],
                examples: vec![
                    "checker(scale=2, even=[1,1,1], odd=[0,0,0]);".to_owned(),
                    "checker(scale=0.5, even=white, odd=black);".to_owned(),
                    "checker(scale=1, even=texture1, odd=texture2);".to_owned(),
                ],
            },
        );

        map.insert(
            "perlin_turbulence",
            ModuleDocs {
                description: "Creates a texture with Perlin turbulence noise effect.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "scale".to_owned(),
                        description: "The scale factor for the Perlin noise pattern.".to_owned(),
                        default: Some("1".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "turbulence_depth".to_owned(),
                        description: "The depth/intensity of the turbulence effect.".to_owned(),
                        default: Some("1".to_owned()),
                    },
                ],
                examples: vec![
                    "perlin_turbulence(1.0, 5);".to_owned(),
                    "perlin_turbulence(scale=2.0, turbulence_depth=3);".to_owned(),
                    "perlin_turbulence(0.5, 7);".to_owned(),
                ],
            },
        );

        map.insert(
            "image",
            ModuleDocs {
                description: "Creates a image texture from a file.".to_owned(),
                arguments: vec![ModuleDocsArguments {
                    name: "filename".to_owned(),
                    description: "path to the image file to render.".to_owned(),
                    default: None,
                }],
                examples: vec!["image(\"photo.png\");".to_owned()],
            },
        );

        map.insert(
            "quad",
            ModuleDocs {
                description: "Creates a 2D rectangle (quadrilateral) at the origin.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "q".to_owned(),
                        description: "position or corner point of the quad.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "u".to_owned(),
                        description: "first direction vector defining quad width and orientation."
                            .to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "v".to_owned(),
                        description:
                            "second direction vector defining quad height and orientation."
                                .to_owned(),
                        default: None,
                    },
                ],
                examples: vec![
                    "quad([0,0], [10,0], [0,20]);".to_owned(),
                    "quad(q=[0,0], u=[10,0], v=[0,20]);".to_owned(),
                    "quad([5,5], [15,0], [0,10]);".to_owned(),
                ],
            },
        );

        // 2D Primitives
        map.insert(
            "circle",
            ModuleDocs {
                description:
                    "Creates a circle at the origin. All parameters, except r, must be named."
                        .to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "r".to_owned(),
                        description: "circle radius. r name is the only one optional with circle."
                            .to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "d".to_owned(),
                        description: "circle diameter.".to_owned(),
                        default: None,
                    },
                ],
                examples: vec![
                    "circle(10);".to_owned(),
                    "circle(r=10);".to_owned(),
                    "circle(d=20);".to_owned(),
                ],
            },
        );

        map.insert(
            "square",
            ModuleDocs {
                description: "Creates a square or rectangle at the origin.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "size".to_owned(),
                        description: "single value for square, or [x,y] for rectangle.".to_owned(),
                        default: Some("1".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "center".to_owned(),
                        description: "if true, centers the square at origin.".to_owned(),
                        default: Some("false".to_owned()),
                    },
                ],
                examples: vec![
                    "square(10);".to_owned(),
                    "square([10, 20]);".to_owned(),
                    "square(10, center=true);".to_owned(),
                ],
            },
        );

        map.insert(
            "polygon",
            ModuleDocs {
                description: "Creates a polygon from a list of points.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "points".to_owned(),
                        description: "list of 2D points [[x,y], ...].".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "paths".to_owned(),
                        description: "optional list of point indices for multiple paths."
                            .to_owned(),
                        default: None,
                    },
                ],
                examples: vec![
                    "polygon(points=[[0,0], [10,0], [10,10], [0,10]]);".to_owned(),
                    "polygon(points=[[0,0], [10,0], [5,10]], paths=[[0,1,2]]);".to_owned(),
                ],
            },
        );

        map.insert(
            "text",
            ModuleDocs {
                description: "Creates text as a 2D geometric object.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "text".to_owned(),
                        description: "the text string to display.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "size".to_owned(),
                        description: "height of the text.".to_owned(),
                        default: Some("10".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "font".to_owned(),
                        description: "font name or font:style.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "halign".to_owned(),
                        description: "horizontal alignment: left, center, or right.".to_owned(),
                        default: Some("left".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "valign".to_owned(),
                        description: "vertical alignment: top, center, baseline, or bottom."
                            .to_owned(),
                        default: Some("baseline".to_owned()),
                    },
                ],
                examples: vec![
                    "text(\"Hello\");".to_owned(),
                    "text(\"OpenSCAD\", size=20, font=\"Arial:Bold\");".to_owned(),
                ],
            },
        );

        // 3D Primitives
        map.insert(
            "cube",
            ModuleDocs {
                description: "Creates a cube or rectangular box at the origin.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "size".to_owned(),
                        description: "single value for cube, or [x,y,z] for box.".to_owned(),
                        default: Some("1".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "center".to_owned(),
                        description: "if true, centers the cube at origin.".to_owned(),
                        default: Some("false".to_owned()),
                    },
                ],
                examples: vec![
                    "cube(10);".to_owned(),
                    "cube([10, 20, 30]);".to_owned(),
                    "cube(10, center=true);".to_owned(),
                ],
            },
        );

        map.insert(
            "sphere",
            ModuleDocs {
                description: "Creates a sphere at the origin.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "r".to_owned(),
                        description: "sphere radius.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "d".to_owned(),
                        description: "sphere diameter.".to_owned(),
                        default: None,
                    },
                ],
                examples: vec![
                    "sphere(10);".to_owned(),
                    "sphere(r=10);".to_owned(),
                    "sphere(d=20);".to_owned(),
                ],
            },
        );

        map.insert(
            "cylinder",
            ModuleDocs {
                description: "Creates a cylinder or cone.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "h".to_owned(),
                        description: "height of the cylinder.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "r".to_owned(),
                        description: "radius of cylinder (or both ends for cone).".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "r1".to_owned(),
                        description: "bottom radius (for cone).".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "r2".to_owned(),
                        description: "top radius (for cone).".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "d".to_owned(),
                        description: "diameter of cylinder.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "d1".to_owned(),
                        description: "bottom diameter (for cone).".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "d2".to_owned(),
                        description: "top diameter (for cone).".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "center".to_owned(),
                        description: "if true, centers cylinder vertically.".to_owned(),
                        default: Some("false".to_owned()),
                    },
                ],
                examples: vec![
                    "cylinder(h=10, r=5);".to_owned(),
                    "cylinder(h=10, d=10);".to_owned(),
                    "cylinder(h=10, r1=5, r2=2);".to_owned(),
                    "cylinder(h=10, r=5, center=true);".to_owned(),
                ],
            },
        );

        map.insert(
        "polyhedron",
        ModuleDocs {
            description: "Creates a polyhedron from a list of points and faces."
                .to_owned(),
            arguments: vec![
                ModuleDocsArguments {
                    name: "points".to_owned(),
                    description: "list of 3D points [[x,y,z], ...].".to_owned(),
                    default: None,
                },
                ModuleDocsArguments {
                    name: "faces".to_owned(),
                    description: "list of faces, each face is a list of point indices.".to_owned(),
                    default: None,
                },
                ModuleDocsArguments {
                    name: "convexity".to_owned(),
                    description: "number of ray crossings for correct rendering.".to_owned(),
                    default: Some("1".to_owned()),
                },
            ],
            examples: vec![
                "polyhedron(points=[[0,0,0], [10,0,0], [0,10,0], [0,0,10]], faces=[[0,1,2], [0,1,3], [0,2,3], [1,2,3]]);".to_owned(),
            ],
        },
    );

        // Transformations
        map.insert(
            "translate",
            ModuleDocs {
                description: "Translates (moves) its child elements along the specified vector."
                    .to_owned(),
                arguments: vec![ModuleDocsArguments {
                    name: "v".to_owned(),
                    description: "vector to translate shape along [x, y, z] or [x, y].".to_owned(),
                    default: None,
                }],
                examples: vec![
                    "translate([10, 20, 30]) { ... }".to_owned(),
                    "translate(v = [x, y, z]) { ... }".to_owned(),
                ],
            },
        );

        map.insert(
            "rotate",
            ModuleDocs {
                description: "Rotates its child elements around the axis.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "a".to_owned(),
                        description:
                            "angle in degrees, or [x, y, z] for rotation around each axis."
                                .to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "v".to_owned(),
                        description: "vector defining the axis of rotation.".to_owned(),
                        default: None,
                    },
                ],
                examples: vec![
                    "rotate([0, 0, 45]) { ... }".to_owned(),
                    "rotate(a=45, v=[0, 0, 1]) { ... }".to_owned(),
                    "rotate(45) { ... }".to_owned(),
                ],
            },
        );

        map.insert(
            "scale",
            ModuleDocs {
                description: "Scales its child elements by the specified factors.".to_owned(),
                arguments: vec![ModuleDocsArguments {
                    name: "v".to_owned(),
                    description:
                        "scaling factors [x, y, z] or [x, y], or single value for uniform scaling."
                            .to_owned(),
                    default: None,
                }],
                examples: vec![
                    "scale([2, 1, 1]) { ... }".to_owned(),
                    "scale(2) { ... }".to_owned(),
                    "scale(v = [x, y, z]) { ... }".to_owned(),
                ],
            },
        );

        map.insert(
            "resize",
            ModuleDocs {
                description: "Resizes child objects to match the specified dimensions.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "newsize".to_owned(),
                        description: "new size [x, y, z] or [x, y]. Use 0 for auto.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "auto".to_owned(),
                        description: "auto-scale [x, y, z] or single bool.".to_owned(),
                        default: None,
                    },
                ],
                examples: vec![
                    "resize([10, 20, 30]) { ... }".to_owned(),
                    "resize([100, 0, 0], auto=true) { ... }".to_owned(),
                ],
            },
        );

        map.insert(
            "mirror",
            ModuleDocs {
                description: "Mirrors child elements across a plane through the origin.".to_owned(),
                arguments: vec![ModuleDocsArguments {
                    name: "v".to_owned(),
                    description: "normal vector of the mirror plane [x, y, z] or [x, y]."
                        .to_owned(),
                    default: None,
                }],
                examples: vec![
                    "mirror([1, 0, 0]) { ... }".to_owned(),
                    "mirror([0, 1, 0]) { ... }".to_owned(),
                ],
            },
        );

        map.insert(
            "multmatrix",
            ModuleDocs {
                description: "Multiplies child elements by an arbitrary 4x4 transformation matrix."
                    .to_owned(),
                arguments: vec![ModuleDocsArguments {
                    name: "m".to_owned(),
                    description: "4x4 transformation matrix.".to_owned(),
                    default: None,
                }],
                examples: vec![
                    "multmatrix([[1,0,0,10], [0,1,0,20], [0,0,1,30], [0,0,0,1]]) { ... }"
                        .to_owned(),
                ],
            },
        );

        map.insert(
            "color",
            ModuleDocs {
                description: "Sets the color of child elements.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "c".to_owned(),
                        description:
                            "color as [r, g, b] or [r, g, b, a] (0-1), or color name string."
                                .to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "alpha".to_owned(),
                        description: "opacity value (0-1).".to_owned(),
                        default: None,
                    },
                ],
                examples: vec![
                    "color(\"red\") { ... }".to_owned(),
                    "color([1, 0, 0]) { ... }".to_owned(),
                    "color([1, 0, 0, 0.5]) { ... }".to_owned(),
                    "color(\"blue\", alpha=0.5) { ... }".to_owned(),
                ],
            },
        );

        map.insert(
            "offset",
            ModuleDocs {
                description: "Offsets 2D shapes inward or outward by a specified amount."
                    .to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "r".to_owned(),
                        description: "offset radius (positive for outward, negative for inward)."
                            .to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "delta".to_owned(),
                        description: "offset distance (alternative to r).".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "chamfer".to_owned(),
                        description: "if true, creates chamfered corners instead of round."
                            .to_owned(),
                        default: Some("false".to_owned()),
                    },
                ],
                examples: vec![
                    "offset(r=5) { ... }".to_owned(),
                    "offset(delta=-2) { ... }".to_owned(),
                    "offset(r=3, chamfer=true) { ... }".to_owned(),
                ],
            },
        );

        // Boolean Operations
        map.insert(
            "union",
            ModuleDocs {
                description: "Creates the union (sum) of all child elements.".to_owned(),
                arguments: vec![],
                examples: vec!["union() { cube(10); translate([5,5,0]) cube(10); }".to_owned()],
            },
        );

        map.insert(
            "difference",
            ModuleDocs {
                description: "Subtracts all children after the first from the first child."
                    .to_owned(),
                arguments: vec![],
                examples: vec![
                    "difference() { cube(10); translate([5,5,5]) sphere(3); }".to_owned(),
                ],
            },
        );

        map.insert(
            "intersection",
            ModuleDocs {
                description: "Creates the intersection (common volume) of all child elements."
                    .to_owned(),
                arguments: vec![],
                examples: vec![
                    "intersection() { cube(10); translate([5,5,5]) sphere(8); }".to_owned(),
                ],
            },
        );

        map.insert(
            "hull",
            ModuleDocs {
                description: "Creates the convex hull of all child elements.".to_owned(),
                arguments: vec![],
                examples: vec![
                    "hull() { translate([0,0,0]) sphere(5); translate([10,10,0]) sphere(5); }"
                        .to_owned(),
                ],
            },
        );

        map.insert(
            "minkowski",
            ModuleDocs {
                description: "Creates the Minkowski sum of child elements.".to_owned(),
                arguments: vec![ModuleDocsArguments {
                    name: "convexity".to_owned(),
                    description: "number for correct rendering of concave objects.".to_owned(),
                    default: None,
                }],
                examples: vec!["minkowski() { cube(10); sphere(2); }".to_owned()],
            },
        );

        // Extrusion Operations
        map.insert(
            "linear_extrude",
            ModuleDocs {
                description: "Extrudes a 2D shape linearly into 3D space.".to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "height".to_owned(),
                        description: "height of the extrusion.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "center".to_owned(),
                        description: "if true, centers the extrusion vertically.".to_owned(),
                        default: Some("false".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "convexity".to_owned(),
                        description: "number for correct rendering.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "twist".to_owned(),
                        description: "degrees of twist along height.".to_owned(),
                        default: Some("0".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "slices".to_owned(),
                        description: "number of slices for twist.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "scale".to_owned(),
                        description: "scale factor at the top.".to_owned(),
                        default: Some("1".to_owned()),
                    },
                ],
                examples: vec![
                    "linear_extrude(height=10) { circle(5); }".to_owned(),
                    "linear_extrude(height=20, twist=90, slices=20) { square(10); }".to_owned(),
                    "linear_extrude(height=10, scale=0.5) { circle(5); }".to_owned(),
                ],
            },
        );

        map.insert(
            "rotate_extrude",
            ModuleDocs {
                description: "Rotates a 2D shape around the Z-axis to create a 3D object."
                    .to_owned(),
                arguments: vec![
                    ModuleDocsArguments {
                        name: "angle".to_owned(),
                        description: "angle in degrees (default 360 for full rotation).".to_owned(),
                        default: Some("360".to_owned()),
                    },
                    ModuleDocsArguments {
                        name: "convexity".to_owned(),
                        description: "number for correct rendering.".to_owned(),
                        default: None,
                    },
                    ModuleDocsArguments {
                        name: "$fn".to_owned(),
                        description: "number of fragments.".to_owned(),
                        default: None,
                    },
                ],
                examples: vec![
                    "rotate_extrude() { translate([10,0,0]) circle(2); }".to_owned(),
                    "rotate_extrude(angle=270, $fn=100) { translate([20,0]) square([5,10]); }"
                        .to_owned(),
                ],
            },
        );

        map.insert(
            "projection",
            ModuleDocs {
                description: "Projects a 3D object onto the XY plane to create a 2D shape."
                    .to_owned(),
                arguments: vec![ModuleDocsArguments {
                    name: "cut".to_owned(),
                    description: "if true, only projects the cross-section at z=0.".to_owned(),
                    default: Some("false".to_owned()),
                }],
                examples: vec![
                    "projection() { sphere(10); }".to_owned(),
                    "projection(cut=true) { cube(20); }".to_owned(),
                ],
            },
        );

        map
    },
);
