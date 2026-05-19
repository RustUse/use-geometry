#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod prelude;

#[cfg(feature = "affine")]
pub use use_affine as affine;
#[cfg(feature = "affine")]
pub use use_affine::*;

#[cfg(feature = "angle")]
pub use use_angle as angle;
#[cfg(feature = "angle")]
pub use use_angle::*;

#[cfg(feature = "archimedean")]
pub use use_archimedean as archimedean;
#[cfg(feature = "archimedean")]
pub use use_archimedean::*;

#[cfg(feature = "bezier")]
pub use use_bezier as bezier;
#[cfg(feature = "bezier")]
pub use use_bezier::*;

#[cfg(feature = "bounds")]
pub use use_bounds as bounds;
#[cfg(feature = "bounds")]
pub use use_bounds::*;

#[cfg(feature = "catalan-solid")]
pub use use_catalan_solid as catalan_solid;
#[cfg(feature = "catalan-solid")]
pub use use_catalan_solid::*;

#[cfg(feature = "cell")]
pub use use_cell as cell;
#[cfg(feature = "cell")]
pub use use_cell::*;

#[cfg(feature = "circle")]
pub use use_circle as circle;
#[cfg(feature = "circle")]
pub use use_circle::*;

#[cfg(feature = "complex")]
pub use use_complex as complex;
#[cfg(feature = "complex")]
pub use use_complex::*;

#[cfg(feature = "configuration")]
pub use use_configuration as configuration;
#[cfg(feature = "configuration")]
pub use use_configuration::*;

#[cfg(feature = "congruence")]
pub use use_congruence as congruence;
#[cfg(feature = "congruence")]
pub use use_congruence::*;

#[cfg(feature = "conic")]
pub use use_conic as conic;
#[cfg(feature = "conic")]
pub use use_conic::*;

#[cfg(feature = "containment")]
pub use use_containment as containment;
#[cfg(feature = "containment")]
pub use use_containment::*;

#[cfg(feature = "coordinate")]
pub use use_coordinate as coordinate;
#[cfg(feature = "coordinate")]
pub use use_coordinate::*;

#[cfg(feature = "coxeter")]
pub use use_coxeter as coxeter;
#[cfg(feature = "coxeter")]
pub use use_coxeter::*;

#[cfg(feature = "crease")]
pub use use_crease as crease;
#[cfg(feature = "crease")]
pub use use_crease::*;

#[cfg(feature = "curve")]
pub use use_curve as curve;
#[cfg(feature = "curve")]
pub use use_curve::*;

#[cfg(feature = "delaunay")]
pub use use_delaunay as delaunay;
#[cfg(feature = "delaunay")]
pub use use_delaunay::*;

#[cfg(feature = "dihedral")]
pub use use_dihedral as dihedral;
#[cfg(feature = "dihedral")]
pub use use_dihedral::*;

#[cfg(feature = "dimension")]
pub use use_dimension as dimension;
#[cfg(feature = "dimension")]
pub use use_dimension::*;

#[cfg(feature = "distance")]
pub use use_distance as distance;
#[cfg(feature = "distance")]
pub use use_distance::*;

#[cfg(feature = "duality")]
pub use use_duality as duality;
#[cfg(feature = "duality")]
pub use use_duality::*;

#[cfg(feature = "face")]
pub use use_face as face;
#[cfg(feature = "face")]
pub use use_face::*;

#[cfg(feature = "folding")]
pub use use_folding as folding;
#[cfg(feature = "folding")]
pub use use_folding::*;

#[cfg(feature = "hull")]
pub use use_hull as hull;
#[cfg(feature = "hull")]
pub use use_hull::*;

#[cfg(feature = "hyperplane")]
pub use use_hyperplane as hyperplane;
#[cfg(feature = "hyperplane")]
pub use use_hyperplane::*;

#[cfg(feature = "hypersphere")]
pub use use_hypersphere as hypersphere;
#[cfg(feature = "hypersphere")]
pub use use_hypersphere::*;

#[cfg(feature = "incidence")]
pub use use_incidence as incidence;
#[cfg(feature = "incidence")]
pub use use_incidence::*;

#[cfg(feature = "intersection")]
pub use use_intersection as intersection;
#[cfg(feature = "intersection")]
pub use use_intersection::*;

#[cfg(feature = "inversion")]
pub use use_inversion as inversion;
#[cfg(feature = "inversion")]
pub use use_inversion::*;

#[cfg(feature = "johnson-solid")]
pub use use_johnson_solid as johnson_solid;
#[cfg(feature = "johnson-solid")]
pub use use_johnson_solid::*;

#[cfg(feature = "line")]
pub use use_line as line;
#[cfg(feature = "line")]
pub use use_line::*;

#[cfg(feature = "linkage")]
pub use use_linkage as linkage;
#[cfg(feature = "linkage")]
pub use use_linkage::*;

#[cfg(feature = "manifold")]
pub use use_manifold as manifold;
#[cfg(feature = "manifold")]
pub use use_manifold::*;

#[cfg(feature = "mesh")]
pub use use_mesh as mesh;
#[cfg(feature = "mesh")]
pub use use_mesh::*;

#[cfg(feature = "orientation")]
pub use use_orientation as orientation;
#[cfg(feature = "orientation")]
pub use use_orientation::*;

#[cfg(feature = "origami")]
pub use use_origami as origami;
#[cfg(feature = "origami")]
pub use use_origami::*;

#[cfg(feature = "orthotope")]
pub use use_orthotope as orthotope;
#[cfg(feature = "orthotope")]
pub use use_orthotope::*;

#[cfg(feature = "plane")]
pub use use_plane as plane;
#[cfg(feature = "plane")]
pub use use_plane::*;

#[cfg(feature = "point")]
pub use use_point as point;
#[cfg(feature = "point")]
pub use use_point::*;

#[cfg(feature = "polygon")]
pub use use_polygon as polygon;
#[cfg(feature = "polygon")]
pub use use_polygon::*;

#[cfg(feature = "polyhedral-net")]
pub use use_polyhedral_net as polyhedral_net;
#[cfg(feature = "polyhedral-net")]
pub use use_polyhedral_net::*;

#[cfg(feature = "polyhedron")]
pub use use_polyhedron as polyhedron;
#[cfg(feature = "polyhedron")]
pub use use_polyhedron::*;

#[cfg(feature = "polyline")]
pub use use_polyline as polyline;
#[cfg(feature = "polyline")]
pub use use_polyline::*;

#[cfg(feature = "polytope")]
pub use use_polytope as polytope;
#[cfg(feature = "polytope")]
pub use use_polytope::*;

#[cfg(feature = "projection")]
pub use use_projection as projection;
#[cfg(feature = "projection")]
pub use use_projection::*;

#[cfg(feature = "projective")]
pub use use_projective as projective;
#[cfg(feature = "projective")]
pub use use_projective::*;

#[cfg(feature = "ray")]
pub use use_ray as ray;
#[cfg(feature = "ray")]
pub use use_ray::*;

#[cfg(feature = "rectangle")]
pub use use_rectangle as rectangle;
#[cfg(feature = "rectangle")]
pub use use_rectangle::*;

#[cfg(feature = "reflection")]
pub use use_reflection as reflection;
#[cfg(feature = "reflection")]
pub use use_reflection::*;

#[cfg(feature = "regular-polytope")]
pub use use_regular_polytope as regular_polytope;
#[cfg(feature = "regular-polytope")]
pub use use_regular_polytope::*;

#[cfg(feature = "rigid-folding")]
pub use use_rigid_folding as rigid_folding;
#[cfg(feature = "rigid-folding")]
pub use use_rigid_folding::*;

#[cfg(feature = "schlafli")]
pub use use_schlafli as schlafli;
#[cfg(feature = "schlafli")]
pub use use_schlafli::*;

#[cfg(feature = "segment")]
pub use use_segment as segment;
#[cfg(feature = "segment")]
pub use use_segment::*;

#[cfg(feature = "similarity")]
pub use use_similarity as similarity;
#[cfg(feature = "similarity")]
pub use use_similarity::*;

#[cfg(feature = "simplex")]
pub use use_simplex as simplex;
#[cfg(feature = "simplex")]
pub use use_simplex::*;

#[cfg(feature = "sphere")]
pub use use_sphere as sphere;
#[cfg(feature = "sphere")]
pub use use_sphere::*;

#[cfg(feature = "spline")]
pub use use_spline as spline;
#[cfg(feature = "spline")]
pub use use_spline::*;

#[cfg(feature = "surface")]
pub use use_surface as surface;
#[cfg(feature = "surface")]
pub use use_surface::*;

#[cfg(feature = "tessellation")]
pub use use_tessellation as tessellation;
#[cfg(feature = "tessellation")]
pub use use_tessellation::*;

#[cfg(feature = "torus")]
pub use use_torus as torus;
#[cfg(feature = "torus")]
pub use use_torus::*;

#[cfg(feature = "transform")]
pub use use_transform as transform;
#[cfg(feature = "transform")]
pub use use_transform::*;

#[cfg(feature = "triangle")]
pub use use_triangle as triangle;
#[cfg(feature = "triangle")]
pub use use_triangle::*;

#[cfg(feature = "triangulation")]
pub use use_triangulation as triangulation;
#[cfg(feature = "triangulation")]
pub use use_triangulation::*;

#[cfg(feature = "unfolding")]
pub use use_unfolding as unfolding;
#[cfg(feature = "unfolding")]
pub use use_unfolding::*;

#[cfg(feature = "uniform-polytope")]
pub use use_uniform_polytope as uniform_polytope;
#[cfg(feature = "uniform-polytope")]
pub use use_uniform_polytope::*;

#[cfg(feature = "voronoi")]
pub use use_voronoi as voronoi;
#[cfg(feature = "voronoi")]
pub use use_voronoi::*;

#[cfg(feature = "wythoff")]
pub use use_wythoff as wythoff;
#[cfg(feature = "wythoff")]
pub use use_wythoff::*;
