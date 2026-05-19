use use_geometry::{folding, mesh, point, polyhedral_net, regular_polytope, schlafli};

#[test]
fn representative_feature_modules_and_root_exports_are_available() {
    let point = point::Point2::new(1.0, 2.0);
    let root_point: use_geometry::Point2 = point;
    assert_eq!(root_point, use_geometry::Point2::new(1.0, 2.0));

    let symbol = schlafli::SchlafliSymbol::polychoron(3, 4, 3).expect("valid symbol");
    let root_symbol: use_geometry::SchlafliSymbol = symbol.clone();
    assert_eq!(root_symbol.to_string(), "{3, 4, 3}");

    let polytope = regular_polytope::RegularPolytope4::TwentyFourCell;
    let root_polytope: use_geometry::RegularPolytope4 = polytope;
    assert_eq!(root_polytope.schlafli_symbol().to_string(), "{3, 4, 3}");

    let mesh = mesh::Mesh::new(8, 12, 6).expect("valid mesh");
    let root_mesh: use_geometry::Mesh = mesh;
    assert_eq!(root_mesh.face_count(), 6);

    let assignment = folding::FoldAssignment::Mountain;
    let root_assignment: use_geometry::FoldAssignment = assignment;
    assert_eq!(root_assignment, use_geometry::FoldAssignment::Mountain);

    let edge = polyhedral_net::NetEdge::new(0, 1);
    let root_edge: use_geometry::NetEdge = edge;
    assert_eq!(root_edge.faces(), (0, 1));
}
