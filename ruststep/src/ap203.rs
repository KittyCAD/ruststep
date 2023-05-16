#![allow(dead_code)]
pub mod config_control_design {
    use crate::{as_holder, derive_more::*, primitive::*, Holder, TableInit};
    use std::collections::HashMap;
    #[derive(Debug, Clone, PartialEq, Default, TableInit)]
    pub struct Tables {
        action: HashMap<u64, as_holder!(Action)>,
        action_assignment: HashMap<u64, as_holder!(ActionAssignment)>,
        action_directive: HashMap<u64, as_holder!(ActionDirective)>,
        action_method: HashMap<u64, as_holder!(ActionMethod)>,
        action_request_assignment: HashMap<u64, as_holder!(ActionRequestAssignment)>,
        action_request_solution: HashMap<u64, as_holder!(ActionRequestSolution)>,
        action_request_status: HashMap<u64, as_holder!(ActionRequestStatus)>,
        action_status: HashMap<u64, as_holder!(ActionStatus)>,
        address: HashMap<u64, as_holder!(Address)>,
        advanced_brep_shape_representation:
            HashMap<u64, as_holder!(AdvancedBrepShapeRepresentation)>,
        advanced_face: HashMap<u64, as_holder!(AdvancedFace)>,
        alternate_product_relationship: HashMap<u64, as_holder!(AlternateProductRelationship)>,
        application_context: HashMap<u64, as_holder!(ApplicationContext)>,
        application_context_element: HashMap<u64, as_holder!(ApplicationContextElement)>,
        application_protocol_definition: HashMap<u64, as_holder!(ApplicationProtocolDefinition)>,
        approval: HashMap<u64, as_holder!(Approval)>,
        approval_assignment: HashMap<u64, as_holder!(ApprovalAssignment)>,
        approval_date_time: HashMap<u64, as_holder!(ApprovalDateTime)>,
        approval_person_organization: HashMap<u64, as_holder!(ApprovalPersonOrganization)>,
        approval_relationship: HashMap<u64, as_holder!(ApprovalRelationship)>,
        approval_role: HashMap<u64, as_holder!(ApprovalRole)>,
        approval_status: HashMap<u64, as_holder!(ApprovalStatus)>,
        area_measure_with_unit: HashMap<u64, as_holder!(AreaMeasureWithUnit)>,
        area_unit: HashMap<u64, as_holder!(AreaUnit)>,
        assembly_component_usage: HashMap<u64, as_holder!(AssemblyComponentUsage)>,
        assembly_component_usage_substitute:
            HashMap<u64, as_holder!(AssemblyComponentUsageSubstitute)>,
        axis1_placement: HashMap<u64, as_holder!(Axis1Placement)>,
        axis2_placement_2d: HashMap<u64, as_holder!(Axis2Placement2D)>,
        axis2_placement_3d: HashMap<u64, as_holder!(Axis2Placement3D)>,
        b_spline_curve: HashMap<u64, as_holder!(BSplineCurve)>,
        b_spline_curve_with_knots: HashMap<u64, as_holder!(BSplineCurveWithKnots)>,
        b_spline_surface: HashMap<u64, as_holder!(BSplineSurface)>,
        b_spline_surface_with_knots: HashMap<u64, as_holder!(BSplineSurfaceWithKnots)>,
        bezier_curve: HashMap<u64, as_holder!(BezierCurve)>,
        bezier_surface: HashMap<u64, as_holder!(BezierSurface)>,
        boundary_curve: HashMap<u64, as_holder!(BoundaryCurve)>,
        bounded_curve: HashMap<u64, as_holder!(BoundedCurve)>,
        bounded_pcurve: HashMap<u64, as_holder!(BoundedPcurve)>,
        bounded_surface: HashMap<u64, as_holder!(BoundedSurface)>,
        bounded_surface_curve: HashMap<u64, as_holder!(BoundedSurfaceCurve)>,
        brep_with_voids: HashMap<u64, as_holder!(BrepWithVoids)>,
        calendar_date: HashMap<u64, as_holder!(CalendarDate)>,
        cartesian_point: HashMap<u64, as_holder!(CartesianPoint)>,
        cartesian_transformation_operator:
            HashMap<u64, as_holder!(CartesianTransformationOperator)>,
        cartesian_transformation_operator_3d:
            HashMap<u64, as_holder!(CartesianTransformationOperator3D)>,
        cc_design_approval: HashMap<u64, as_holder!(CcDesignApproval)>,
        cc_design_certification: HashMap<u64, as_holder!(CcDesignCertification)>,
        cc_design_contract: HashMap<u64, as_holder!(CcDesignContract)>,
        cc_design_date_and_time_assignment: HashMap<u64, as_holder!(CcDesignDateAndTimeAssignment)>,
        cc_design_person_and_organization_assignment:
            HashMap<u64, as_holder!(CcDesignPersonAndOrganizationAssignment)>,
        cc_design_security_classification: HashMap<u64, as_holder!(CcDesignSecurityClassification)>,
        cc_design_specification_reference: HashMap<u64, as_holder!(CcDesignSpecificationReference)>,
        certification: HashMap<u64, as_holder!(Certification)>,
        certification_assignment: HashMap<u64, as_holder!(CertificationAssignment)>,
        certification_type: HashMap<u64, as_holder!(CertificationType)>,
        change: HashMap<u64, as_holder!(Change)>,
        change_request: HashMap<u64, as_holder!(ChangeRequest)>,
        circle: HashMap<u64, as_holder!(Circle)>,
        closed_shell: HashMap<u64, as_holder!(ClosedShell)>,
        composite_curve: HashMap<u64, as_holder!(CompositeCurve)>,
        composite_curve_on_surface: HashMap<u64, as_holder!(CompositeCurveOnSurface)>,
        composite_curve_segment: HashMap<u64, as_holder!(CompositeCurveSegment)>,
        configuration_design: HashMap<u64, as_holder!(ConfigurationDesign)>,
        configuration_effectivity: HashMap<u64, as_holder!(ConfigurationEffectivity)>,
        configuration_item: HashMap<u64, as_holder!(ConfigurationItem)>,
        conic: HashMap<u64, as_holder!(Conic)>,
        conical_surface: HashMap<u64, as_holder!(ConicalSurface)>,
        connected_edge_set: HashMap<u64, as_holder!(ConnectedEdgeSet)>,
        connected_face_set: HashMap<u64, as_holder!(ConnectedFaceSet)>,
        context_dependent_shape_representation:
            HashMap<u64, as_holder!(ContextDependentShapeRepresentation)>,
        context_dependent_unit: HashMap<u64, as_holder!(ContextDependentUnit)>,
        contract: HashMap<u64, as_holder!(Contract)>,
        contract_assignment: HashMap<u64, as_holder!(ContractAssignment)>,
        contract_type: HashMap<u64, as_holder!(ContractType)>,
        conversion_based_unit: HashMap<u64, as_holder!(ConversionBasedUnit)>,
        coordinated_universal_time_offset: HashMap<u64, as_holder!(CoordinatedUniversalTimeOffset)>,
        curve: HashMap<u64, as_holder!(Curve)>,
        curve_bounded_surface: HashMap<u64, as_holder!(CurveBoundedSurface)>,
        curve_replica: HashMap<u64, as_holder!(CurveReplica)>,
        cylindrical_surface: HashMap<u64, as_holder!(CylindricalSurface)>,
        date: HashMap<u64, as_holder!(Date)>,
        date_and_time: HashMap<u64, as_holder!(DateAndTime)>,
        date_and_time_assignment: HashMap<u64, as_holder!(DateAndTimeAssignment)>,
        date_time_role: HashMap<u64, as_holder!(DateTimeRole)>,
        dated_effectivity: HashMap<u64, as_holder!(DatedEffectivity)>,
        definitional_representation: HashMap<u64, as_holder!(DefinitionalRepresentation)>,
        degenerate_pcurve: HashMap<u64, as_holder!(DegeneratePcurve)>,
        degenerate_toroidal_surface: HashMap<u64, as_holder!(DegenerateToroidalSurface)>,
        design_context: HashMap<u64, as_holder!(DesignContext)>,
        design_make_from_relationship: HashMap<u64, as_holder!(DesignMakeFromRelationship)>,
        dimensional_exponents: HashMap<u64, as_holder!(DimensionalExponents)>,
        directed_action: HashMap<u64, as_holder!(DirectedAction)>,
        direction: HashMap<u64, as_holder!(Direction)>,
        document: HashMap<u64, as_holder!(Document)>,
        document_reference: HashMap<u64, as_holder!(DocumentReference)>,
        document_relationship: HashMap<u64, as_holder!(DocumentRelationship)>,
        document_type: HashMap<u64, as_holder!(DocumentType)>,
        document_usage_constraint: HashMap<u64, as_holder!(DocumentUsageConstraint)>,
        document_with_class: HashMap<u64, as_holder!(DocumentWithClass)>,
        edge: HashMap<u64, as_holder!(Edge)>,
        edge_based_wireframe_model: HashMap<u64, as_holder!(EdgeBasedWireframeModel)>,
        edge_based_wireframe_shape_representation:
            HashMap<u64, as_holder!(EdgeBasedWireframeShapeRepresentation)>,
        edge_curve: HashMap<u64, as_holder!(EdgeCurve)>,
        edge_loop: HashMap<u64, as_holder!(EdgeLoop)>,
        effectivity: HashMap<u64, as_holder!(Effectivity)>,
        elementary_surface: HashMap<u64, as_holder!(ElementarySurface)>,
        ellipse: HashMap<u64, as_holder!(Ellipse)>,
        evaluated_degenerate_pcurve: HashMap<u64, as_holder!(EvaluatedDegeneratePcurve)>,
        executed_action: HashMap<u64, as_holder!(ExecutedAction)>,
        face: HashMap<u64, as_holder!(Face)>,
        face_bound: HashMap<u64, as_holder!(FaceBound)>,
        face_outer_bound: HashMap<u64, as_holder!(FaceOuterBound)>,
        face_surface: HashMap<u64, as_holder!(FaceSurface)>,
        faceted_brep: HashMap<u64, as_holder!(FacetedBrep)>,
        faceted_brep_shape_representation: HashMap<u64, as_holder!(FacetedBrepShapeRepresentation)>,
        founded_item: HashMap<u64, as_holder!(FoundedItem)>,
        functionally_defined_transformation:
            HashMap<u64, as_holder!(FunctionallyDefinedTransformation)>,
        geometric_curve_set: HashMap<u64, as_holder!(GeometricCurveSet)>,
        geometric_representation_context: HashMap<u64, as_holder!(GeometricRepresentationContext)>,
        geometric_representation_item: HashMap<u64, as_holder!(GeometricRepresentationItem)>,
        geometric_set: HashMap<u64, as_holder!(GeometricSet)>,
        geometrically_bounded_surface_shape_representation:
            HashMap<u64, as_holder!(GeometricallyBoundedSurfaceShapeRepresentation)>,
        geometrically_bounded_wireframe_shape_representation:
            HashMap<u64, as_holder!(GeometricallyBoundedWireframeShapeRepresentation)>,
        global_uncertainty_assigned_context:
            HashMap<u64, as_holder!(GlobalUncertaintyAssignedContext)>,
        global_unit_assigned_context: HashMap<u64, as_holder!(GlobalUnitAssignedContext)>,
        hyperbola: HashMap<u64, as_holder!(Hyperbola)>,
        intersection_curve: HashMap<u64, as_holder!(IntersectionCurve)>,
        item_defined_transformation: HashMap<u64, as_holder!(ItemDefinedTransformation)>,
        length_measure_with_unit: HashMap<u64, as_holder!(LengthMeasureWithUnit)>,
        length_unit: HashMap<u64, as_holder!(LengthUnit)>,
        line: HashMap<u64, as_holder!(Line)>,
        local_time: HashMap<u64, as_holder!(LocalTime)>,
        r#loop: HashMap<u64, as_holder!(Loop)>,
        lot_effectivity: HashMap<u64, as_holder!(LotEffectivity)>,
        manifold_solid_brep: HashMap<u64, as_holder!(ManifoldSolidBrep)>,
        manifold_surface_shape_representation:
            HashMap<u64, as_holder!(ManifoldSurfaceShapeRepresentation)>,
        mapped_item: HashMap<u64, as_holder!(MappedItem)>,
        mass_measure_with_unit: HashMap<u64, as_holder!(MassMeasureWithUnit)>,
        mass_unit: HashMap<u64, as_holder!(MassUnit)>,
        measure_with_unit: HashMap<u64, as_holder!(MeasureWithUnit)>,
        mechanical_context: HashMap<u64, as_holder!(MechanicalContext)>,
        named_unit: HashMap<u64, as_holder!(NamedUnit)>,
        next_assembly_usage_occurrence: HashMap<u64, as_holder!(NextAssemblyUsageOccurrence)>,
        offset_curve_3d: HashMap<u64, as_holder!(OffsetCurve3D)>,
        offset_surface: HashMap<u64, as_holder!(OffsetSurface)>,
        open_shell: HashMap<u64, as_holder!(OpenShell)>,
        ordinal_date: HashMap<u64, as_holder!(OrdinalDate)>,
        organization: HashMap<u64, as_holder!(Organization)>,
        organization_relationship: HashMap<u64, as_holder!(OrganizationRelationship)>,
        organizational_address: HashMap<u64, as_holder!(OrganizationalAddress)>,
        organizational_project: HashMap<u64, as_holder!(OrganizationalProject)>,
        oriented_closed_shell: HashMap<u64, as_holder!(OrientedClosedShell)>,
        oriented_edge: HashMap<u64, as_holder!(OrientedEdge)>,
        oriented_face: HashMap<u64, as_holder!(OrientedFace)>,
        oriented_open_shell: HashMap<u64, as_holder!(OrientedOpenShell)>,
        oriented_path: HashMap<u64, as_holder!(OrientedPath)>,
        outer_boundary_curve: HashMap<u64, as_holder!(OuterBoundaryCurve)>,
        parabola: HashMap<u64, as_holder!(Parabola)>,
        parametric_representation_context:
            HashMap<u64, as_holder!(ParametricRepresentationContext)>,
        path: HashMap<u64, as_holder!(Path)>,
        pcurve: HashMap<u64, as_holder!(Pcurve)>,
        person: HashMap<u64, as_holder!(Person)>,
        person_and_organization: HashMap<u64, as_holder!(PersonAndOrganization)>,
        person_and_organization_assignment:
            HashMap<u64, as_holder!(PersonAndOrganizationAssignment)>,
        person_and_organization_role: HashMap<u64, as_holder!(PersonAndOrganizationRole)>,
        personal_address: HashMap<u64, as_holder!(PersonalAddress)>,
        placement: HashMap<u64, as_holder!(Placement)>,
        plane: HashMap<u64, as_holder!(Plane)>,
        plane_angle_measure_with_unit: HashMap<u64, as_holder!(PlaneAngleMeasureWithUnit)>,
        plane_angle_unit: HashMap<u64, as_holder!(PlaneAngleUnit)>,
        point: HashMap<u64, as_holder!(Point)>,
        point_on_curve: HashMap<u64, as_holder!(PointOnCurve)>,
        point_on_surface: HashMap<u64, as_holder!(PointOnSurface)>,
        point_replica: HashMap<u64, as_holder!(PointReplica)>,
        poly_loop: HashMap<u64, as_holder!(PolyLoop)>,
        polyline: HashMap<u64, as_holder!(Polyline)>,
        product: HashMap<u64, as_holder!(Product)>,
        product_category: HashMap<u64, as_holder!(ProductCategory)>,
        product_category_relationship: HashMap<u64, as_holder!(ProductCategoryRelationship)>,
        product_concept: HashMap<u64, as_holder!(ProductConcept)>,
        product_concept_context: HashMap<u64, as_holder!(ProductConceptContext)>,
        product_context: HashMap<u64, as_holder!(ProductContext)>,
        product_definition: HashMap<u64, as_holder!(ProductDefinition)>,
        product_definition_context: HashMap<u64, as_holder!(ProductDefinitionContext)>,
        product_definition_effectivity: HashMap<u64, as_holder!(ProductDefinitionEffectivity)>,
        product_definition_formation: HashMap<u64, as_holder!(ProductDefinitionFormation)>,
        product_definition_formation_with_specified_source:
            HashMap<u64, as_holder!(ProductDefinitionFormationWithSpecifiedSource)>,
        product_definition_relationship: HashMap<u64, as_holder!(ProductDefinitionRelationship)>,
        product_definition_shape: HashMap<u64, as_holder!(ProductDefinitionShape)>,
        product_definition_usage: HashMap<u64, as_holder!(ProductDefinitionUsage)>,
        product_definition_with_associated_documents:
            HashMap<u64, as_holder!(ProductDefinitionWithAssociatedDocuments)>,
        product_related_product_category: HashMap<u64, as_holder!(ProductRelatedProductCategory)>,
        promissory_usage_occurrence: HashMap<u64, as_holder!(PromissoryUsageOccurrence)>,
        property_definition: HashMap<u64, as_holder!(PropertyDefinition)>,
        property_definition_representation:
            HashMap<u64, as_holder!(PropertyDefinitionRepresentation)>,
        quantified_assembly_component_usage:
            HashMap<u64, as_holder!(QuantifiedAssemblyComponentUsage)>,
        quasi_uniform_curve: HashMap<u64, as_holder!(QuasiUniformCurve)>,
        quasi_uniform_surface: HashMap<u64, as_holder!(QuasiUniformSurface)>,
        rational_b_spline_curve: HashMap<u64, as_holder!(RationalBSplineCurve)>,
        rational_b_spline_surface: HashMap<u64, as_holder!(RationalBSplineSurface)>,
        rectangular_composite_surface: HashMap<u64, as_holder!(RectangularCompositeSurface)>,
        rectangular_trimmed_surface: HashMap<u64, as_holder!(RectangularTrimmedSurface)>,
        reparametrised_composite_curve_segment:
            HashMap<u64, as_holder!(ReparametrisedCompositeCurveSegment)>,
        representation: HashMap<u64, as_holder!(Representation)>,
        representation_context: HashMap<u64, as_holder!(RepresentationContext)>,
        representation_item: HashMap<u64, as_holder!(RepresentationItem)>,
        representation_map: HashMap<u64, as_holder!(RepresentationMap)>,
        representation_relationship: HashMap<u64, as_holder!(RepresentationRelationship)>,
        representation_relationship_with_transformation:
            HashMap<u64, as_holder!(RepresentationRelationshipWithTransformation)>,
        seam_curve: HashMap<u64, as_holder!(SeamCurve)>,
        security_classification: HashMap<u64, as_holder!(SecurityClassification)>,
        security_classification_assignment:
            HashMap<u64, as_holder!(SecurityClassificationAssignment)>,
        security_classification_level: HashMap<u64, as_holder!(SecurityClassificationLevel)>,
        serial_numbered_effectivity: HashMap<u64, as_holder!(SerialNumberedEffectivity)>,
        shape_aspect: HashMap<u64, as_holder!(ShapeAspect)>,
        shape_aspect_relationship: HashMap<u64, as_holder!(ShapeAspectRelationship)>,
        shape_definition_representation: HashMap<u64, as_holder!(ShapeDefinitionRepresentation)>,
        shape_representation: HashMap<u64, as_holder!(ShapeRepresentation)>,
        shape_representation_relationship:
            HashMap<u64, as_holder!(ShapeRepresentationRelationship)>,
        shell_based_surface_model: HashMap<u64, as_holder!(ShellBasedSurfaceModel)>,
        shell_based_wireframe_model: HashMap<u64, as_holder!(ShellBasedWireframeModel)>,
        shell_based_wireframe_shape_representation:
            HashMap<u64, as_holder!(ShellBasedWireframeShapeRepresentation)>,
        si_unit: HashMap<u64, as_holder!(SiUnit)>,
        solid_angle_measure_with_unit: HashMap<u64, as_holder!(SolidAngleMeasureWithUnit)>,
        solid_angle_unit: HashMap<u64, as_holder!(SolidAngleUnit)>,
        solid_model: HashMap<u64, as_holder!(SolidModel)>,
        specified_higher_usage_occurrence: HashMap<u64, as_holder!(SpecifiedHigherUsageOccurrence)>,
        spherical_surface: HashMap<u64, as_holder!(SphericalSurface)>,
        start_request: HashMap<u64, as_holder!(StartRequest)>,
        start_work: HashMap<u64, as_holder!(StartWork)>,
        supplied_part_relationship: HashMap<u64, as_holder!(SuppliedPartRelationship)>,
        surface: HashMap<u64, as_holder!(Surface)>,
        surface_curve: HashMap<u64, as_holder!(SurfaceCurve)>,
        surface_of_linear_extrusion: HashMap<u64, as_holder!(SurfaceOfLinearExtrusion)>,
        surface_of_revolution: HashMap<u64, as_holder!(SurfaceOfRevolution)>,
        surface_patch: HashMap<u64, as_holder!(SurfacePatch)>,
        surface_replica: HashMap<u64, as_holder!(SurfaceReplica)>,
        swept_surface: HashMap<u64, as_holder!(SweptSurface)>,
        topological_representation_item: HashMap<u64, as_holder!(TopologicalRepresentationItem)>,
        toroidal_surface: HashMap<u64, as_holder!(ToroidalSurface)>,
        trimmed_curve: HashMap<u64, as_holder!(TrimmedCurve)>,
        uncertainty_measure_with_unit: HashMap<u64, as_holder!(UncertaintyMeasureWithUnit)>,
        uniform_curve: HashMap<u64, as_holder!(UniformCurve)>,
        uniform_surface: HashMap<u64, as_holder!(UniformSurface)>,
        vector: HashMap<u64, as_holder!(Vector)>,
        versioned_action_request: HashMap<u64, as_holder!(VersionedActionRequest)>,
        vertex: HashMap<u64, as_holder!(Vertex)>,
        vertex_loop: HashMap<u64, as_holder!(VertexLoop)>,
        vertex_point: HashMap<u64, as_holder!(VertexPoint)>,
        vertex_shell: HashMap<u64, as_holder!(VertexShell)>,
        volume_measure_with_unit: HashMap<u64, as_holder!(VolumeMeasureWithUnit)>,
        volume_unit: HashMap<u64, as_holder!(VolumeUnit)>,
        week_of_year_and_day_date: HashMap<u64, as_holder!(WeekOfYearAndDayDate)>,
        wire_shell: HashMap<u64, as_holder!(WireShell)>,
        approved_item: HashMap<u64, as_holder!(ApprovedItem)>,
        area_measure: HashMap<u64, as_holder!(AreaMeasure)>,
        axis2_placement: HashMap<u64, as_holder!(Axis2Placement)>,
        boolean_operand: HashMap<u64, as_holder!(BooleanOperand)>,
        certified_item: HashMap<u64, as_holder!(CertifiedItem)>,
        change_request_item: HashMap<u64, as_holder!(ChangeRequestItem)>,
        characterized_definition: HashMap<u64, as_holder!(CharacterizedDefinition)>,
        characterized_product_definition: HashMap<u64, as_holder!(CharacterizedProductDefinition)>,
        classified_item: HashMap<u64, as_holder!(ClassifiedItem)>,
        context_dependent_measure: HashMap<u64, as_holder!(ContextDependentMeasure)>,
        contracted_item: HashMap<u64, as_holder!(ContractedItem)>,
        count_measure: HashMap<u64, as_holder!(CountMeasure)>,
        curve_on_surface: HashMap<u64, as_holder!(CurveOnSurface)>,
        date_time_item: HashMap<u64, as_holder!(DateTimeItem)>,
        date_time_select: HashMap<u64, as_holder!(DateTimeSelect)>,
        day_in_month_number: HashMap<u64, as_holder!(DayInMonthNumber)>,
        day_in_week_number: HashMap<u64, as_holder!(DayInWeekNumber)>,
        day_in_year_number: HashMap<u64, as_holder!(DayInYearNumber)>,
        descriptive_measure: HashMap<u64, as_holder!(DescriptiveMeasure)>,
        dimension_count: HashMap<u64, as_holder!(DimensionCount)>,
        founded_item_select: HashMap<u64, as_holder!(FoundedItemSelect)>,
        geometric_set_select: HashMap<u64, as_holder!(GeometricSetSelect)>,
        hour_in_day: HashMap<u64, as_holder!(HourInDay)>,
        identifier: HashMap<u64, as_holder!(Identifier)>,
        label: HashMap<u64, as_holder!(Label)>,
        length_measure: HashMap<u64, as_holder!(LengthMeasure)>,
        list_of_reversible_topology_item: HashMap<u64, as_holder!(ListOfReversibleTopologyItem)>,
        mass_measure: HashMap<u64, as_holder!(MassMeasure)>,
        measure_value: HashMap<u64, as_holder!(MeasureValue)>,
        minute_in_hour: HashMap<u64, as_holder!(MinuteInHour)>,
        month_in_year_number: HashMap<u64, as_holder!(MonthInYearNumber)>,
        parameter_value: HashMap<u64, as_holder!(ParameterValue)>,
        pcurve_or_surface: HashMap<u64, as_holder!(PcurveOrSurface)>,
        person_organization_item: HashMap<u64, as_holder!(PersonOrganizationItem)>,
        person_organization_select: HashMap<u64, as_holder!(PersonOrganizationSelect)>,
        plane_angle_measure: HashMap<u64, as_holder!(PlaneAngleMeasure)>,
        positive_length_measure: HashMap<u64, as_holder!(PositiveLengthMeasure)>,
        positive_plane_angle_measure: HashMap<u64, as_holder!(PositivePlaneAngleMeasure)>,
        reversible_topology: HashMap<u64, as_holder!(ReversibleTopology)>,
        reversible_topology_item: HashMap<u64, as_holder!(ReversibleTopologyItem)>,
        second_in_minute: HashMap<u64, as_holder!(SecondInMinute)>,
        set_of_reversible_topology_item: HashMap<u64, as_holder!(SetOfReversibleTopologyItem)>,
        shape_definition: HashMap<u64, as_holder!(ShapeDefinition)>,
        shell: HashMap<u64, as_holder!(Shell)>,
        solid_angle_measure: HashMap<u64, as_holder!(SolidAngleMeasure)>,
        specified_item: HashMap<u64, as_holder!(SpecifiedItem)>,
        start_request_item: HashMap<u64, as_holder!(StartRequestItem)>,
        supported_item: HashMap<u64, as_holder!(SupportedItem)>,
        surface_model: HashMap<u64, as_holder!(SurfaceModel)>,
        text: HashMap<u64, as_holder!(Text)>,
        transformation: HashMap<u64, as_holder!(Transformation)>,
        trimming_select: HashMap<u64, as_holder!(TrimmingSelect)>,
        unit: HashMap<u64, as_holder!(Unit)>,
        vector_or_direction: HashMap<u64, as_holder!(VectorOrDirection)>,
        volume_measure: HashMap<u64, as_holder!(VolumeMeasure)>,
        week_in_year_number: HashMap<u64, as_holder!(WeekInYearNumber)>,
        wireframe_model: HashMap<u64, as_holder!(WireframeModel)>,
        work_item: HashMap<u64, as_holder!(WorkItem)>,
        year_number: HashMap<u64, as_holder!(YearNumber)>,
    }
    impl Tables {
        pub fn action_holders(&self) -> &HashMap<u64, as_holder!(Action)> {
            &self.action
        }
        pub fn action_assignment_holders(&self) -> &HashMap<u64, as_holder!(ActionAssignment)> {
            &self.action_assignment
        }
        pub fn action_directive_holders(&self) -> &HashMap<u64, as_holder!(ActionDirective)> {
            &self.action_directive
        }
        pub fn action_method_holders(&self) -> &HashMap<u64, as_holder!(ActionMethod)> {
            &self.action_method
        }
        pub fn action_request_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ActionRequestAssignment)> {
            &self.action_request_assignment
        }
        pub fn action_request_solution_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ActionRequestSolution)> {
            &self.action_request_solution
        }
        pub fn action_request_status_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ActionRequestStatus)> {
            &self.action_request_status
        }
        pub fn action_status_holders(&self) -> &HashMap<u64, as_holder!(ActionStatus)> {
            &self.action_status
        }
        pub fn address_holders(&self) -> &HashMap<u64, as_holder!(Address)> {
            &self.address
        }
        pub fn advanced_brep_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AdvancedBrepShapeRepresentation)> {
            &self.advanced_brep_shape_representation
        }
        pub fn advanced_face_holders(&self) -> &HashMap<u64, as_holder!(AdvancedFace)> {
            &self.advanced_face
        }
        pub fn alternate_product_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AlternateProductRelationship)> {
            &self.alternate_product_relationship
        }
        pub fn application_context_holders(&self) -> &HashMap<u64, as_holder!(ApplicationContext)> {
            &self.application_context
        }
        pub fn application_context_element_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ApplicationContextElement)> {
            &self.application_context_element
        }
        pub fn application_protocol_definition_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ApplicationProtocolDefinition)> {
            &self.application_protocol_definition
        }
        pub fn approval_holders(&self) -> &HashMap<u64, as_holder!(Approval)> {
            &self.approval
        }
        pub fn approval_assignment_holders(&self) -> &HashMap<u64, as_holder!(ApprovalAssignment)> {
            &self.approval_assignment
        }
        pub fn approval_date_time_holders(&self) -> &HashMap<u64, as_holder!(ApprovalDateTime)> {
            &self.approval_date_time
        }
        pub fn approval_person_organization_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ApprovalPersonOrganization)> {
            &self.approval_person_organization
        }
        pub fn approval_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ApprovalRelationship)> {
            &self.approval_relationship
        }
        pub fn approval_role_holders(&self) -> &HashMap<u64, as_holder!(ApprovalRole)> {
            &self.approval_role
        }
        pub fn approval_status_holders(&self) -> &HashMap<u64, as_holder!(ApprovalStatus)> {
            &self.approval_status
        }
        pub fn area_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AreaMeasureWithUnit)> {
            &self.area_measure_with_unit
        }
        pub fn area_unit_holders(&self) -> &HashMap<u64, as_holder!(AreaUnit)> {
            &self.area_unit
        }
        pub fn assembly_component_usage_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AssemblyComponentUsage)> {
            &self.assembly_component_usage
        }
        pub fn assembly_component_usage_substitute_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AssemblyComponentUsageSubstitute)> {
            &self.assembly_component_usage_substitute
        }
        pub fn axis1_placement_holders(&self) -> &HashMap<u64, as_holder!(Axis1Placement)> {
            &self.axis1_placement
        }
        pub fn axis2_placement_2d_holders(&self) -> &HashMap<u64, as_holder!(Axis2Placement2D)> {
            &self.axis2_placement_2d
        }
        pub fn axis2_placement_3d_holders(&self) -> &HashMap<u64, as_holder!(Axis2Placement3D)> {
            &self.axis2_placement_3d
        }
        pub fn b_spline_curve_holders(&self) -> &HashMap<u64, as_holder!(BSplineCurve)> {
            &self.b_spline_curve
        }
        pub fn b_spline_curve_with_knots_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(BSplineCurveWithKnots)> {
            &self.b_spline_curve_with_knots
        }
        pub fn b_spline_surface_holders(&self) -> &HashMap<u64, as_holder!(BSplineSurface)> {
            &self.b_spline_surface
        }
        pub fn b_spline_surface_with_knots_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(BSplineSurfaceWithKnots)> {
            &self.b_spline_surface_with_knots
        }
        pub fn bezier_curve_holders(&self) -> &HashMap<u64, as_holder!(BezierCurve)> {
            &self.bezier_curve
        }
        pub fn bezier_surface_holders(&self) -> &HashMap<u64, as_holder!(BezierSurface)> {
            &self.bezier_surface
        }
        pub fn boundary_curve_holders(&self) -> &HashMap<u64, as_holder!(BoundaryCurve)> {
            &self.boundary_curve
        }
        pub fn bounded_curve_holders(&self) -> &HashMap<u64, as_holder!(BoundedCurve)> {
            &self.bounded_curve
        }
        pub fn bounded_pcurve_holders(&self) -> &HashMap<u64, as_holder!(BoundedPcurve)> {
            &self.bounded_pcurve
        }
        pub fn bounded_surface_holders(&self) -> &HashMap<u64, as_holder!(BoundedSurface)> {
            &self.bounded_surface
        }
        pub fn bounded_surface_curve_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(BoundedSurfaceCurve)> {
            &self.bounded_surface_curve
        }
        pub fn brep_with_voids_holders(&self) -> &HashMap<u64, as_holder!(BrepWithVoids)> {
            &self.brep_with_voids
        }
        pub fn calendar_date_holders(&self) -> &HashMap<u64, as_holder!(CalendarDate)> {
            &self.calendar_date
        }
        pub fn cartesian_point_holders(&self) -> &HashMap<u64, as_holder!(CartesianPoint)> {
            &self.cartesian_point
        }
        pub fn cartesian_transformation_operator_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CartesianTransformationOperator)> {
            &self.cartesian_transformation_operator
        }
        pub fn cartesian_transformation_operator_3d_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CartesianTransformationOperator3D)> {
            &self.cartesian_transformation_operator_3d
        }
        pub fn cc_design_approval_holders(&self) -> &HashMap<u64, as_holder!(CcDesignApproval)> {
            &self.cc_design_approval
        }
        pub fn cc_design_certification_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CcDesignCertification)> {
            &self.cc_design_certification
        }
        pub fn cc_design_contract_holders(&self) -> &HashMap<u64, as_holder!(CcDesignContract)> {
            &self.cc_design_contract
        }
        pub fn cc_design_date_and_time_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CcDesignDateAndTimeAssignment)> {
            &self.cc_design_date_and_time_assignment
        }
        pub fn cc_design_person_and_organization_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CcDesignPersonAndOrganizationAssignment)> {
            &self.cc_design_person_and_organization_assignment
        }
        pub fn cc_design_security_classification_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CcDesignSecurityClassification)> {
            &self.cc_design_security_classification
        }
        pub fn cc_design_specification_reference_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CcDesignSpecificationReference)> {
            &self.cc_design_specification_reference
        }
        pub fn certification_holders(&self) -> &HashMap<u64, as_holder!(Certification)> {
            &self.certification
        }
        pub fn certification_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CertificationAssignment)> {
            &self.certification_assignment
        }
        pub fn certification_type_holders(&self) -> &HashMap<u64, as_holder!(CertificationType)> {
            &self.certification_type
        }
        pub fn change_holders(&self) -> &HashMap<u64, as_holder!(Change)> {
            &self.change
        }
        pub fn change_request_holders(&self) -> &HashMap<u64, as_holder!(ChangeRequest)> {
            &self.change_request
        }
        pub fn circle_holders(&self) -> &HashMap<u64, as_holder!(Circle)> {
            &self.circle
        }
        pub fn closed_shell_holders(&self) -> &HashMap<u64, as_holder!(ClosedShell)> {
            &self.closed_shell
        }
        pub fn composite_curve_holders(&self) -> &HashMap<u64, as_holder!(CompositeCurve)> {
            &self.composite_curve
        }
        pub fn composite_curve_on_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CompositeCurveOnSurface)> {
            &self.composite_curve_on_surface
        }
        pub fn composite_curve_segment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CompositeCurveSegment)> {
            &self.composite_curve_segment
        }
        pub fn configuration_design_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ConfigurationDesign)> {
            &self.configuration_design
        }
        pub fn configuration_effectivity_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ConfigurationEffectivity)> {
            &self.configuration_effectivity
        }
        pub fn configuration_item_holders(&self) -> &HashMap<u64, as_holder!(ConfigurationItem)> {
            &self.configuration_item
        }
        pub fn conic_holders(&self) -> &HashMap<u64, as_holder!(Conic)> {
            &self.conic
        }
        pub fn conical_surface_holders(&self) -> &HashMap<u64, as_holder!(ConicalSurface)> {
            &self.conical_surface
        }
        pub fn connected_edge_set_holders(&self) -> &HashMap<u64, as_holder!(ConnectedEdgeSet)> {
            &self.connected_edge_set
        }
        pub fn connected_face_set_holders(&self) -> &HashMap<u64, as_holder!(ConnectedFaceSet)> {
            &self.connected_face_set
        }
        pub fn context_dependent_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ContextDependentShapeRepresentation)> {
            &self.context_dependent_shape_representation
        }
        pub fn context_dependent_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ContextDependentUnit)> {
            &self.context_dependent_unit
        }
        pub fn contract_holders(&self) -> &HashMap<u64, as_holder!(Contract)> {
            &self.contract
        }
        pub fn contract_assignment_holders(&self) -> &HashMap<u64, as_holder!(ContractAssignment)> {
            &self.contract_assignment
        }
        pub fn contract_type_holders(&self) -> &HashMap<u64, as_holder!(ContractType)> {
            &self.contract_type
        }
        pub fn conversion_based_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ConversionBasedUnit)> {
            &self.conversion_based_unit
        }
        pub fn coordinated_universal_time_offset_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CoordinatedUniversalTimeOffset)> {
            &self.coordinated_universal_time_offset
        }
        pub fn curve_holders(&self) -> &HashMap<u64, as_holder!(Curve)> {
            &self.curve
        }
        pub fn curve_bounded_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CurveBoundedSurface)> {
            &self.curve_bounded_surface
        }
        pub fn curve_replica_holders(&self) -> &HashMap<u64, as_holder!(CurveReplica)> {
            &self.curve_replica
        }
        pub fn cylindrical_surface_holders(&self) -> &HashMap<u64, as_holder!(CylindricalSurface)> {
            &self.cylindrical_surface
        }
        pub fn date_holders(&self) -> &HashMap<u64, as_holder!(Date)> {
            &self.date
        }
        pub fn date_and_time_holders(&self) -> &HashMap<u64, as_holder!(DateAndTime)> {
            &self.date_and_time
        }
        pub fn date_and_time_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DateAndTimeAssignment)> {
            &self.date_and_time_assignment
        }
        pub fn date_time_role_holders(&self) -> &HashMap<u64, as_holder!(DateTimeRole)> {
            &self.date_time_role
        }
        pub fn dated_effectivity_holders(&self) -> &HashMap<u64, as_holder!(DatedEffectivity)> {
            &self.dated_effectivity
        }
        pub fn definitional_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DefinitionalRepresentation)> {
            &self.definitional_representation
        }
        pub fn degenerate_pcurve_holders(&self) -> &HashMap<u64, as_holder!(DegeneratePcurve)> {
            &self.degenerate_pcurve
        }
        pub fn degenerate_toroidal_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DegenerateToroidalSurface)> {
            &self.degenerate_toroidal_surface
        }
        pub fn design_context_holders(&self) -> &HashMap<u64, as_holder!(DesignContext)> {
            &self.design_context
        }
        pub fn design_make_from_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DesignMakeFromRelationship)> {
            &self.design_make_from_relationship
        }
        pub fn dimensional_exponents_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DimensionalExponents)> {
            &self.dimensional_exponents
        }
        pub fn directed_action_holders(&self) -> &HashMap<u64, as_holder!(DirectedAction)> {
            &self.directed_action
        }
        pub fn direction_holders(&self) -> &HashMap<u64, as_holder!(Direction)> {
            &self.direction
        }
        pub fn document_holders(&self) -> &HashMap<u64, as_holder!(Document)> {
            &self.document
        }
        pub fn document_reference_holders(&self) -> &HashMap<u64, as_holder!(DocumentReference)> {
            &self.document_reference
        }
        pub fn document_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DocumentRelationship)> {
            &self.document_relationship
        }
        pub fn document_type_holders(&self) -> &HashMap<u64, as_holder!(DocumentType)> {
            &self.document_type
        }
        pub fn document_usage_constraint_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DocumentUsageConstraint)> {
            &self.document_usage_constraint
        }
        pub fn document_with_class_holders(&self) -> &HashMap<u64, as_holder!(DocumentWithClass)> {
            &self.document_with_class
        }
        pub fn edge_holders(&self) -> &HashMap<u64, as_holder!(Edge)> {
            &self.edge
        }
        pub fn edge_based_wireframe_model_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(EdgeBasedWireframeModel)> {
            &self.edge_based_wireframe_model
        }
        pub fn edge_based_wireframe_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(EdgeBasedWireframeShapeRepresentation)> {
            &self.edge_based_wireframe_shape_representation
        }
        pub fn edge_curve_holders(&self) -> &HashMap<u64, as_holder!(EdgeCurve)> {
            &self.edge_curve
        }
        pub fn edge_loop_holders(&self) -> &HashMap<u64, as_holder!(EdgeLoop)> {
            &self.edge_loop
        }
        pub fn effectivity_holders(&self) -> &HashMap<u64, as_holder!(Effectivity)> {
            &self.effectivity
        }
        pub fn elementary_surface_holders(&self) -> &HashMap<u64, as_holder!(ElementarySurface)> {
            &self.elementary_surface
        }
        pub fn ellipse_holders(&self) -> &HashMap<u64, as_holder!(Ellipse)> {
            &self.ellipse
        }
        pub fn evaluated_degenerate_pcurve_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(EvaluatedDegeneratePcurve)> {
            &self.evaluated_degenerate_pcurve
        }
        pub fn executed_action_holders(&self) -> &HashMap<u64, as_holder!(ExecutedAction)> {
            &self.executed_action
        }
        pub fn face_holders(&self) -> &HashMap<u64, as_holder!(Face)> {
            &self.face
        }
        pub fn face_bound_holders(&self) -> &HashMap<u64, as_holder!(FaceBound)> {
            &self.face_bound
        }
        pub fn face_outer_bound_holders(&self) -> &HashMap<u64, as_holder!(FaceOuterBound)> {
            &self.face_outer_bound
        }
        pub fn face_surface_holders(&self) -> &HashMap<u64, as_holder!(FaceSurface)> {
            &self.face_surface
        }
        pub fn faceted_brep_holders(&self) -> &HashMap<u64, as_holder!(FacetedBrep)> {
            &self.faceted_brep
        }
        pub fn faceted_brep_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(FacetedBrepShapeRepresentation)> {
            &self.faceted_brep_shape_representation
        }
        pub fn founded_item_holders(&self) -> &HashMap<u64, as_holder!(FoundedItem)> {
            &self.founded_item
        }
        pub fn functionally_defined_transformation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(FunctionallyDefinedTransformation)> {
            &self.functionally_defined_transformation
        }
        pub fn geometric_curve_set_holders(&self) -> &HashMap<u64, as_holder!(GeometricCurveSet)> {
            &self.geometric_curve_set
        }
        pub fn geometric_representation_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricRepresentationContext)> {
            &self.geometric_representation_context
        }
        pub fn geometric_representation_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricRepresentationItem)> {
            &self.geometric_representation_item
        }
        pub fn geometric_set_holders(&self) -> &HashMap<u64, as_holder!(GeometricSet)> {
            &self.geometric_set
        }
        pub fn geometrically_bounded_surface_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricallyBoundedSurfaceShapeRepresentation)> {
            &self.geometrically_bounded_surface_shape_representation
        }
        pub fn geometrically_bounded_wireframe_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricallyBoundedWireframeShapeRepresentation)> {
            &self.geometrically_bounded_wireframe_shape_representation
        }
        pub fn global_uncertainty_assigned_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GlobalUncertaintyAssignedContext)> {
            &self.global_uncertainty_assigned_context
        }
        pub fn global_unit_assigned_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GlobalUnitAssignedContext)> {
            &self.global_unit_assigned_context
        }
        pub fn hyperbola_holders(&self) -> &HashMap<u64, as_holder!(Hyperbola)> {
            &self.hyperbola
        }
        pub fn intersection_curve_holders(&self) -> &HashMap<u64, as_holder!(IntersectionCurve)> {
            &self.intersection_curve
        }
        pub fn item_defined_transformation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ItemDefinedTransformation)> {
            &self.item_defined_transformation
        }
        pub fn length_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(LengthMeasureWithUnit)> {
            &self.length_measure_with_unit
        }
        pub fn length_unit_holders(&self) -> &HashMap<u64, as_holder!(LengthUnit)> {
            &self.length_unit
        }
        pub fn line_holders(&self) -> &HashMap<u64, as_holder!(Line)> {
            &self.line
        }
        pub fn local_time_holders(&self) -> &HashMap<u64, as_holder!(LocalTime)> {
            &self.local_time
        }
        pub fn loop_holders(&self) -> &HashMap<u64, as_holder!(Loop)> {
            &self.r#loop
        }
        pub fn lot_effectivity_holders(&self) -> &HashMap<u64, as_holder!(LotEffectivity)> {
            &self.lot_effectivity
        }
        pub fn manifold_solid_brep_holders(&self) -> &HashMap<u64, as_holder!(ManifoldSolidBrep)> {
            &self.manifold_solid_brep
        }
        pub fn manifold_surface_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ManifoldSurfaceShapeRepresentation)> {
            &self.manifold_surface_shape_representation
        }
        pub fn mapped_item_holders(&self) -> &HashMap<u64, as_holder!(MappedItem)> {
            &self.mapped_item
        }
        pub fn mass_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(MassMeasureWithUnit)> {
            &self.mass_measure_with_unit
        }
        pub fn mass_unit_holders(&self) -> &HashMap<u64, as_holder!(MassUnit)> {
            &self.mass_unit
        }
        pub fn measure_with_unit_holders(&self) -> &HashMap<u64, as_holder!(MeasureWithUnit)> {
            &self.measure_with_unit
        }
        pub fn mechanical_context_holders(&self) -> &HashMap<u64, as_holder!(MechanicalContext)> {
            &self.mechanical_context
        }
        pub fn named_unit_holders(&self) -> &HashMap<u64, as_holder!(NamedUnit)> {
            &self.named_unit
        }
        pub fn next_assembly_usage_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(NextAssemblyUsageOccurrence)> {
            &self.next_assembly_usage_occurrence
        }
        pub fn offset_curve_3d_holders(&self) -> &HashMap<u64, as_holder!(OffsetCurve3D)> {
            &self.offset_curve_3d
        }
        pub fn offset_surface_holders(&self) -> &HashMap<u64, as_holder!(OffsetSurface)> {
            &self.offset_surface
        }
        pub fn open_shell_holders(&self) -> &HashMap<u64, as_holder!(OpenShell)> {
            &self.open_shell
        }
        pub fn ordinal_date_holders(&self) -> &HashMap<u64, as_holder!(OrdinalDate)> {
            &self.ordinal_date
        }
        pub fn organization_holders(&self) -> &HashMap<u64, as_holder!(Organization)> {
            &self.organization
        }
        pub fn organization_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OrganizationRelationship)> {
            &self.organization_relationship
        }
        pub fn organizational_address_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OrganizationalAddress)> {
            &self.organizational_address
        }
        pub fn organizational_project_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OrganizationalProject)> {
            &self.organizational_project
        }
        pub fn oriented_closed_shell_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OrientedClosedShell)> {
            &self.oriented_closed_shell
        }
        pub fn oriented_edge_holders(&self) -> &HashMap<u64, as_holder!(OrientedEdge)> {
            &self.oriented_edge
        }
        pub fn oriented_face_holders(&self) -> &HashMap<u64, as_holder!(OrientedFace)> {
            &self.oriented_face
        }
        pub fn oriented_open_shell_holders(&self) -> &HashMap<u64, as_holder!(OrientedOpenShell)> {
            &self.oriented_open_shell
        }
        pub fn oriented_path_holders(&self) -> &HashMap<u64, as_holder!(OrientedPath)> {
            &self.oriented_path
        }
        pub fn outer_boundary_curve_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OuterBoundaryCurve)> {
            &self.outer_boundary_curve
        }
        pub fn parabola_holders(&self) -> &HashMap<u64, as_holder!(Parabola)> {
            &self.parabola
        }
        pub fn parametric_representation_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ParametricRepresentationContext)> {
            &self.parametric_representation_context
        }
        pub fn path_holders(&self) -> &HashMap<u64, as_holder!(Path)> {
            &self.path
        }
        pub fn pcurve_holders(&self) -> &HashMap<u64, as_holder!(Pcurve)> {
            &self.pcurve
        }
        pub fn person_holders(&self) -> &HashMap<u64, as_holder!(Person)> {
            &self.person
        }
        pub fn person_and_organization_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PersonAndOrganization)> {
            &self.person_and_organization
        }
        pub fn person_and_organization_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PersonAndOrganizationAssignment)> {
            &self.person_and_organization_assignment
        }
        pub fn person_and_organization_role_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PersonAndOrganizationRole)> {
            &self.person_and_organization_role
        }
        pub fn personal_address_holders(&self) -> &HashMap<u64, as_holder!(PersonalAddress)> {
            &self.personal_address
        }
        pub fn placement_holders(&self) -> &HashMap<u64, as_holder!(Placement)> {
            &self.placement
        }
        pub fn plane_holders(&self) -> &HashMap<u64, as_holder!(Plane)> {
            &self.plane
        }
        pub fn plane_angle_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PlaneAngleMeasureWithUnit)> {
            &self.plane_angle_measure_with_unit
        }
        pub fn plane_angle_unit_holders(&self) -> &HashMap<u64, as_holder!(PlaneAngleUnit)> {
            &self.plane_angle_unit
        }
        pub fn point_holders(&self) -> &HashMap<u64, as_holder!(Point)> {
            &self.point
        }
        pub fn point_on_curve_holders(&self) -> &HashMap<u64, as_holder!(PointOnCurve)> {
            &self.point_on_curve
        }
        pub fn point_on_surface_holders(&self) -> &HashMap<u64, as_holder!(PointOnSurface)> {
            &self.point_on_surface
        }
        pub fn point_replica_holders(&self) -> &HashMap<u64, as_holder!(PointReplica)> {
            &self.point_replica
        }
        pub fn poly_loop_holders(&self) -> &HashMap<u64, as_holder!(PolyLoop)> {
            &self.poly_loop
        }
        pub fn polyline_holders(&self) -> &HashMap<u64, as_holder!(Polyline)> {
            &self.polyline
        }
        pub fn product_holders(&self) -> &HashMap<u64, as_holder!(Product)> {
            &self.product
        }
        pub fn product_category_holders(&self) -> &HashMap<u64, as_holder!(ProductCategory)> {
            &self.product_category
        }
        pub fn product_category_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductCategoryRelationship)> {
            &self.product_category_relationship
        }
        pub fn product_concept_holders(&self) -> &HashMap<u64, as_holder!(ProductConcept)> {
            &self.product_concept
        }
        pub fn product_concept_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductConceptContext)> {
            &self.product_concept_context
        }
        pub fn product_context_holders(&self) -> &HashMap<u64, as_holder!(ProductContext)> {
            &self.product_context
        }
        pub fn product_definition_holders(&self) -> &HashMap<u64, as_holder!(ProductDefinition)> {
            &self.product_definition
        }
        pub fn product_definition_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionContext)> {
            &self.product_definition_context
        }
        pub fn product_definition_effectivity_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionEffectivity)> {
            &self.product_definition_effectivity
        }
        pub fn product_definition_formation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionFormation)> {
            &self.product_definition_formation
        }
        pub fn product_definition_formation_with_specified_source_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionFormationWithSpecifiedSource)> {
            &self.product_definition_formation_with_specified_source
        }
        pub fn product_definition_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionRelationship)> {
            &self.product_definition_relationship
        }
        pub fn product_definition_shape_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionShape)> {
            &self.product_definition_shape
        }
        pub fn product_definition_usage_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionUsage)> {
            &self.product_definition_usage
        }
        pub fn product_definition_with_associated_documents_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionWithAssociatedDocuments)> {
            &self.product_definition_with_associated_documents
        }
        pub fn product_related_product_category_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductRelatedProductCategory)> {
            &self.product_related_product_category
        }
        pub fn promissory_usage_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PromissoryUsageOccurrence)> {
            &self.promissory_usage_occurrence
        }
        pub fn property_definition_holders(&self) -> &HashMap<u64, as_holder!(PropertyDefinition)> {
            &self.property_definition
        }
        pub fn property_definition_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PropertyDefinitionRepresentation)> {
            &self.property_definition_representation
        }
        pub fn quantified_assembly_component_usage_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(QuantifiedAssemblyComponentUsage)> {
            &self.quantified_assembly_component_usage
        }
        pub fn quasi_uniform_curve_holders(&self) -> &HashMap<u64, as_holder!(QuasiUniformCurve)> {
            &self.quasi_uniform_curve
        }
        pub fn quasi_uniform_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(QuasiUniformSurface)> {
            &self.quasi_uniform_surface
        }
        pub fn rational_b_spline_curve_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RationalBSplineCurve)> {
            &self.rational_b_spline_curve
        }
        pub fn rational_b_spline_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RationalBSplineSurface)> {
            &self.rational_b_spline_surface
        }
        pub fn rectangular_composite_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RectangularCompositeSurface)> {
            &self.rectangular_composite_surface
        }
        pub fn rectangular_trimmed_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RectangularTrimmedSurface)> {
            &self.rectangular_trimmed_surface
        }
        pub fn reparametrised_composite_curve_segment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ReparametrisedCompositeCurveSegment)> {
            &self.reparametrised_composite_curve_segment
        }
        pub fn representation_holders(&self) -> &HashMap<u64, as_holder!(Representation)> {
            &self.representation
        }
        pub fn representation_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RepresentationContext)> {
            &self.representation_context
        }
        pub fn representation_item_holders(&self) -> &HashMap<u64, as_holder!(RepresentationItem)> {
            &self.representation_item
        }
        pub fn representation_map_holders(&self) -> &HashMap<u64, as_holder!(RepresentationMap)> {
            &self.representation_map
        }
        pub fn representation_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RepresentationRelationship)> {
            &self.representation_relationship
        }
        pub fn representation_relationship_with_transformation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RepresentationRelationshipWithTransformation)> {
            &self.representation_relationship_with_transformation
        }
        pub fn seam_curve_holders(&self) -> &HashMap<u64, as_holder!(SeamCurve)> {
            &self.seam_curve
        }
        pub fn security_classification_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SecurityClassification)> {
            &self.security_classification
        }
        pub fn security_classification_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SecurityClassificationAssignment)> {
            &self.security_classification_assignment
        }
        pub fn security_classification_level_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SecurityClassificationLevel)> {
            &self.security_classification_level
        }
        pub fn serial_numbered_effectivity_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SerialNumberedEffectivity)> {
            &self.serial_numbered_effectivity
        }
        pub fn shape_aspect_holders(&self) -> &HashMap<u64, as_holder!(ShapeAspect)> {
            &self.shape_aspect
        }
        pub fn shape_aspect_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShapeAspectRelationship)> {
            &self.shape_aspect_relationship
        }
        pub fn shape_definition_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShapeDefinitionRepresentation)> {
            &self.shape_definition_representation
        }
        pub fn shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShapeRepresentation)> {
            &self.shape_representation
        }
        pub fn shape_representation_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShapeRepresentationRelationship)> {
            &self.shape_representation_relationship
        }
        pub fn shell_based_surface_model_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShellBasedSurfaceModel)> {
            &self.shell_based_surface_model
        }
        pub fn shell_based_wireframe_model_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShellBasedWireframeModel)> {
            &self.shell_based_wireframe_model
        }
        pub fn shell_based_wireframe_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShellBasedWireframeShapeRepresentation)> {
            &self.shell_based_wireframe_shape_representation
        }
        pub fn si_unit_holders(&self) -> &HashMap<u64, as_holder!(SiUnit)> {
            &self.si_unit
        }
        pub fn solid_angle_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SolidAngleMeasureWithUnit)> {
            &self.solid_angle_measure_with_unit
        }
        pub fn solid_angle_unit_holders(&self) -> &HashMap<u64, as_holder!(SolidAngleUnit)> {
            &self.solid_angle_unit
        }
        pub fn solid_model_holders(&self) -> &HashMap<u64, as_holder!(SolidModel)> {
            &self.solid_model
        }
        pub fn specified_higher_usage_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SpecifiedHigherUsageOccurrence)> {
            &self.specified_higher_usage_occurrence
        }
        pub fn spherical_surface_holders(&self) -> &HashMap<u64, as_holder!(SphericalSurface)> {
            &self.spherical_surface
        }
        pub fn start_request_holders(&self) -> &HashMap<u64, as_holder!(StartRequest)> {
            &self.start_request
        }
        pub fn start_work_holders(&self) -> &HashMap<u64, as_holder!(StartWork)> {
            &self.start_work
        }
        pub fn supplied_part_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SuppliedPartRelationship)> {
            &self.supplied_part_relationship
        }
        pub fn surface_holders(&self) -> &HashMap<u64, as_holder!(Surface)> {
            &self.surface
        }
        pub fn surface_curve_holders(&self) -> &HashMap<u64, as_holder!(SurfaceCurve)> {
            &self.surface_curve
        }
        pub fn surface_of_linear_extrusion_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SurfaceOfLinearExtrusion)> {
            &self.surface_of_linear_extrusion
        }
        pub fn surface_of_revolution_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SurfaceOfRevolution)> {
            &self.surface_of_revolution
        }
        pub fn surface_patch_holders(&self) -> &HashMap<u64, as_holder!(SurfacePatch)> {
            &self.surface_patch
        }
        pub fn surface_replica_holders(&self) -> &HashMap<u64, as_holder!(SurfaceReplica)> {
            &self.surface_replica
        }
        pub fn swept_surface_holders(&self) -> &HashMap<u64, as_holder!(SweptSurface)> {
            &self.swept_surface
        }
        pub fn topological_representation_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(TopologicalRepresentationItem)> {
            &self.topological_representation_item
        }
        pub fn toroidal_surface_holders(&self) -> &HashMap<u64, as_holder!(ToroidalSurface)> {
            &self.toroidal_surface
        }
        pub fn trimmed_curve_holders(&self) -> &HashMap<u64, as_holder!(TrimmedCurve)> {
            &self.trimmed_curve
        }
        pub fn uncertainty_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(UncertaintyMeasureWithUnit)> {
            &self.uncertainty_measure_with_unit
        }
        pub fn uniform_curve_holders(&self) -> &HashMap<u64, as_holder!(UniformCurve)> {
            &self.uniform_curve
        }
        pub fn uniform_surface_holders(&self) -> &HashMap<u64, as_holder!(UniformSurface)> {
            &self.uniform_surface
        }
        pub fn vector_holders(&self) -> &HashMap<u64, as_holder!(Vector)> {
            &self.vector
        }
        pub fn versioned_action_request_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(VersionedActionRequest)> {
            &self.versioned_action_request
        }
        pub fn vertex_holders(&self) -> &HashMap<u64, as_holder!(Vertex)> {
            &self.vertex
        }
        pub fn vertex_loop_holders(&self) -> &HashMap<u64, as_holder!(VertexLoop)> {
            &self.vertex_loop
        }
        pub fn vertex_point_holders(&self) -> &HashMap<u64, as_holder!(VertexPoint)> {
            &self.vertex_point
        }
        pub fn vertex_shell_holders(&self) -> &HashMap<u64, as_holder!(VertexShell)> {
            &self.vertex_shell
        }
        pub fn volume_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(VolumeMeasureWithUnit)> {
            &self.volume_measure_with_unit
        }
        pub fn volume_unit_holders(&self) -> &HashMap<u64, as_holder!(VolumeUnit)> {
            &self.volume_unit
        }
        pub fn week_of_year_and_day_date_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(WeekOfYearAndDayDate)> {
            &self.week_of_year_and_day_date
        }
        pub fn wire_shell_holders(&self) -> &HashMap<u64, as_holder!(WireShell)> {
            &self.wire_shell
        }
        pub fn approved_item_holders(&self) -> &HashMap<u64, as_holder!(ApprovedItem)> {
            &self.approved_item
        }
        pub fn area_measure_holders(&self) -> &HashMap<u64, as_holder!(AreaMeasure)> {
            &self.area_measure
        }
        pub fn axis2_placement_holders(&self) -> &HashMap<u64, as_holder!(Axis2Placement)> {
            &self.axis2_placement
        }
        pub fn boolean_operand_holders(&self) -> &HashMap<u64, as_holder!(BooleanOperand)> {
            &self.boolean_operand
        }
        pub fn certified_item_holders(&self) -> &HashMap<u64, as_holder!(CertifiedItem)> {
            &self.certified_item
        }
        pub fn change_request_item_holders(&self) -> &HashMap<u64, as_holder!(ChangeRequestItem)> {
            &self.change_request_item
        }
        pub fn characterized_definition_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CharacterizedDefinition)> {
            &self.characterized_definition
        }
        pub fn characterized_product_definition_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CharacterizedProductDefinition)> {
            &self.characterized_product_definition
        }
        pub fn classified_item_holders(&self) -> &HashMap<u64, as_holder!(ClassifiedItem)> {
            &self.classified_item
        }
        pub fn context_dependent_measure_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ContextDependentMeasure)> {
            &self.context_dependent_measure
        }
        pub fn contracted_item_holders(&self) -> &HashMap<u64, as_holder!(ContractedItem)> {
            &self.contracted_item
        }
        pub fn count_measure_holders(&self) -> &HashMap<u64, as_holder!(CountMeasure)> {
            &self.count_measure
        }
        pub fn curve_on_surface_holders(&self) -> &HashMap<u64, as_holder!(CurveOnSurface)> {
            &self.curve_on_surface
        }
        pub fn date_time_item_holders(&self) -> &HashMap<u64, as_holder!(DateTimeItem)> {
            &self.date_time_item
        }
        pub fn date_time_select_holders(&self) -> &HashMap<u64, as_holder!(DateTimeSelect)> {
            &self.date_time_select
        }
        pub fn day_in_month_number_holders(&self) -> &HashMap<u64, as_holder!(DayInMonthNumber)> {
            &self.day_in_month_number
        }
        pub fn day_in_week_number_holders(&self) -> &HashMap<u64, as_holder!(DayInWeekNumber)> {
            &self.day_in_week_number
        }
        pub fn day_in_year_number_holders(&self) -> &HashMap<u64, as_holder!(DayInYearNumber)> {
            &self.day_in_year_number
        }
        pub fn descriptive_measure_holders(&self) -> &HashMap<u64, as_holder!(DescriptiveMeasure)> {
            &self.descriptive_measure
        }
        pub fn dimension_count_holders(&self) -> &HashMap<u64, as_holder!(DimensionCount)> {
            &self.dimension_count
        }
        pub fn founded_item_select_holders(&self) -> &HashMap<u64, as_holder!(FoundedItemSelect)> {
            &self.founded_item_select
        }
        pub fn geometric_set_select_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricSetSelect)> {
            &self.geometric_set_select
        }
        pub fn hour_in_day_holders(&self) -> &HashMap<u64, as_holder!(HourInDay)> {
            &self.hour_in_day
        }
        pub fn identifier_holders(&self) -> &HashMap<u64, as_holder!(Identifier)> {
            &self.identifier
        }
        pub fn label_holders(&self) -> &HashMap<u64, as_holder!(Label)> {
            &self.label
        }
        pub fn length_measure_holders(&self) -> &HashMap<u64, as_holder!(LengthMeasure)> {
            &self.length_measure
        }
        pub fn list_of_reversible_topology_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ListOfReversibleTopologyItem)> {
            &self.list_of_reversible_topology_item
        }
        pub fn mass_measure_holders(&self) -> &HashMap<u64, as_holder!(MassMeasure)> {
            &self.mass_measure
        }
        pub fn measure_value_holders(&self) -> &HashMap<u64, as_holder!(MeasureValue)> {
            &self.measure_value
        }
        pub fn minute_in_hour_holders(&self) -> &HashMap<u64, as_holder!(MinuteInHour)> {
            &self.minute_in_hour
        }
        pub fn month_in_year_number_holders(&self) -> &HashMap<u64, as_holder!(MonthInYearNumber)> {
            &self.month_in_year_number
        }
        pub fn parameter_value_holders(&self) -> &HashMap<u64, as_holder!(ParameterValue)> {
            &self.parameter_value
        }
        pub fn pcurve_or_surface_holders(&self) -> &HashMap<u64, as_holder!(PcurveOrSurface)> {
            &self.pcurve_or_surface
        }
        pub fn person_organization_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PersonOrganizationItem)> {
            &self.person_organization_item
        }
        pub fn person_organization_select_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PersonOrganizationSelect)> {
            &self.person_organization_select
        }
        pub fn plane_angle_measure_holders(&self) -> &HashMap<u64, as_holder!(PlaneAngleMeasure)> {
            &self.plane_angle_measure
        }
        pub fn positive_length_measure_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PositiveLengthMeasure)> {
            &self.positive_length_measure
        }
        pub fn positive_plane_angle_measure_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PositivePlaneAngleMeasure)> {
            &self.positive_plane_angle_measure
        }
        pub fn reversible_topology_holders(&self) -> &HashMap<u64, as_holder!(ReversibleTopology)> {
            &self.reversible_topology
        }
        pub fn reversible_topology_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ReversibleTopologyItem)> {
            &self.reversible_topology_item
        }
        pub fn second_in_minute_holders(&self) -> &HashMap<u64, as_holder!(SecondInMinute)> {
            &self.second_in_minute
        }
        pub fn set_of_reversible_topology_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SetOfReversibleTopologyItem)> {
            &self.set_of_reversible_topology_item
        }
        pub fn shape_definition_holders(&self) -> &HashMap<u64, as_holder!(ShapeDefinition)> {
            &self.shape_definition
        }
        pub fn shell_holders(&self) -> &HashMap<u64, as_holder!(Shell)> {
            &self.shell
        }
        pub fn solid_angle_measure_holders(&self) -> &HashMap<u64, as_holder!(SolidAngleMeasure)> {
            &self.solid_angle_measure
        }
        pub fn specified_item_holders(&self) -> &HashMap<u64, as_holder!(SpecifiedItem)> {
            &self.specified_item
        }
        pub fn start_request_item_holders(&self) -> &HashMap<u64, as_holder!(StartRequestItem)> {
            &self.start_request_item
        }
        pub fn supported_item_holders(&self) -> &HashMap<u64, as_holder!(SupportedItem)> {
            &self.supported_item
        }
        pub fn surface_model_holders(&self) -> &HashMap<u64, as_holder!(SurfaceModel)> {
            &self.surface_model
        }
        pub fn text_holders(&self) -> &HashMap<u64, as_holder!(Text)> {
            &self.text
        }
        pub fn transformation_holders(&self) -> &HashMap<u64, as_holder!(Transformation)> {
            &self.transformation
        }
        pub fn trimming_select_holders(&self) -> &HashMap<u64, as_holder!(TrimmingSelect)> {
            &self.trimming_select
        }
        pub fn unit_holders(&self) -> &HashMap<u64, as_holder!(Unit)> {
            &self.unit
        }
        pub fn vector_or_direction_holders(&self) -> &HashMap<u64, as_holder!(VectorOrDirection)> {
            &self.vector_or_direction
        }
        pub fn volume_measure_holders(&self) -> &HashMap<u64, as_holder!(VolumeMeasure)> {
            &self.volume_measure
        }
        pub fn week_in_year_number_holders(&self) -> &HashMap<u64, as_holder!(WeekInYearNumber)> {
            &self.week_in_year_number
        }
        pub fn wireframe_model_holders(&self) -> &HashMap<u64, as_holder!(WireframeModel)> {
            &self.wireframe_model
        }
        pub fn work_item_holders(&self) -> &HashMap<u64, as_holder!(WorkItem)> {
            &self.work_item
        }
        pub fn year_number_holders(&self) -> &HashMap<u64, as_holder!(YearNumber)> {
            &self.year_number
        }
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum AheadOrBehind {
        Ahead,
        Behind,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApprovedItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        ConfigurationEffectivity(Box<ConfigurationEffectivity>),
        #[holder(use_place_holder)]
        ConfigurationItem(Box<ConfigurationItem>),
        #[holder(use_place_holder)]
        SecurityClassification(Box<SecurityClassification>),
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        #[holder(use_place_holder)]
        Change(Box<Change>),
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
        #[holder(use_place_holder)]
        Certification(Box<Certification>),
        #[holder(use_place_holder)]
        Contract(Box<Contract>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = area_measure)]
    #[holder(generate_deserialize)]
    pub struct AreaMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Axis2Placement {
        #[holder(use_place_holder)]
        Axis2Placement2D(Box<Axis2Placement2D>),
        #[holder(use_place_holder)]
        Axis2Placement3D(Box<Axis2Placement3D>),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum BSplineCurveForm {
        PolylineForm,
        CircularArc,
        EllipticArc,
        ParabolicArc,
        HyperbolicArc,
        Unspecified,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum BSplineSurfaceForm {
        PlaneSurf,
        CylindricalSurf,
        ConicalSurf,
        SphericalSurf,
        ToroidalSurf,
        SurfOfRevolution,
        RuledSurf,
        GeneralisedCone,
        QuadricSurf,
        SurfOfLinearExtrusion,
        Unspecified,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BooleanOperand {
        #[holder(use_place_holder)]
        SolidModel(SolidModelAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CertifiedItem {
        #[holder(use_place_holder)]
        SuppliedPartRelationship(Box<SuppliedPartRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ChangeRequestItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterizedDefinition {
        #[holder(use_place_holder)]
        CharacterizedProductDefinition(Box<CharacterizedProductDefinition>),
        #[holder(use_place_holder)]
        ShapeDefinition(Box<ShapeDefinition>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterizedProductDefinition {
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        ProductDefinitionRelationship(ProductDefinitionRelationshipAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ClassifiedItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
        #[holder(use_place_holder)]
        AssemblyComponentUsage(AssemblyComponentUsageAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_measure)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ContractedItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = count_measure)]
    #[holder(generate_deserialize)]
    pub struct CountMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveOnSurface {
        #[holder(use_place_holder)]
        Pcurve(PcurveAny),
        #[holder(use_place_holder)]
        SurfaceCurve(SurfaceCurveAny),
        #[holder(use_place_holder)]
        CompositeCurveOnSurface(CompositeCurveOnSurfaceAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateTimeItem {
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
        #[holder(use_place_holder)]
        Change(Box<Change>),
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
        #[holder(use_place_holder)]
        ApprovalPersonOrganization(Box<ApprovalPersonOrganization>),
        #[holder(use_place_holder)]
        Contract(Box<Contract>),
        #[holder(use_place_holder)]
        SecurityClassification(Box<SecurityClassification>),
        #[holder(use_place_holder)]
        Certification(Box<Certification>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateTimeSelect {
        #[holder(use_place_holder)]
        Date(DateAny),
        #[holder(use_place_holder)]
        LocalTime(Box<LocalTime>),
        #[holder(use_place_holder)]
        DateAndTime(Box<DateAndTime>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = day_in_month_number)]
    #[holder(generate_deserialize)]
    pub struct DayInMonthNumber(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = day_in_week_number)]
    #[holder(generate_deserialize)]
    pub struct DayInWeekNumber(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = day_in_year_number)]
    #[holder(generate_deserialize)]
    pub struct DayInYearNumber(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = descriptive_measure)]
    #[holder(generate_deserialize)]
    pub struct DescriptiveMeasure(pub String);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = dimension_count)]
    #[holder(generate_deserialize)]
    pub struct DimensionCount(pub i64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FoundedItemSelect {
        #[holder(use_place_holder)]
        FoundedItem(FoundedItemAny),
        #[holder(use_place_holder)]
        RepresentationItem(RepresentationItemAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricSetSelect {
        #[holder(use_place_holder)]
        Point(PointAny),
        #[holder(use_place_holder)]
        Curve(CurveAny),
        #[holder(use_place_holder)]
        Surface(SurfaceAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = hour_in_day)]
    #[holder(generate_deserialize)]
    pub struct HourInDay(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = identifier)]
    #[holder(generate_deserialize)]
    pub struct Identifier(pub String);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum KnotType {
        UniformKnots,
        Unspecified,
        QuasiUniformKnots,
        PiecewiseBezierKnots,
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = label)]
    #[holder(generate_deserialize)]
    pub struct Label(pub String);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = length_measure)]
    #[holder(generate_deserialize)]
    pub struct LengthMeasure(pub f64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = list_of_reversible_topology_item)]
    #[holder(generate_deserialize)]
    pub struct ListOfReversibleTopologyItem(
        #[holder(use_place_holder)] pub Vec<ReversibleTopologyItem>,
    );
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = mass_measure)]
    #[holder(generate_deserialize)]
    pub struct MassMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MeasureValue {
        #[holder(use_place_holder)]
        LengthMeasure(Box<LengthMeasure>),
        #[holder(use_place_holder)]
        MassMeasure(Box<MassMeasure>),
        #[holder(use_place_holder)]
        PlaneAngleMeasure(Box<PlaneAngleMeasure>),
        #[holder(use_place_holder)]
        SolidAngleMeasure(Box<SolidAngleMeasure>),
        #[holder(use_place_holder)]
        AreaMeasure(Box<AreaMeasure>),
        #[holder(use_place_holder)]
        VolumeMeasure(Box<VolumeMeasure>),
        #[holder(use_place_holder)]
        ParameterValue(Box<ParameterValue>),
        #[holder(use_place_holder)]
        ContextDependentMeasure(Box<ContextDependentMeasure>),
        #[holder(use_place_holder)]
        DescriptiveMeasure(Box<DescriptiveMeasure>),
        #[holder(use_place_holder)]
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        #[holder(use_place_holder)]
        PositivePlaneAngleMeasure(Box<PositivePlaneAngleMeasure>),
        #[holder(use_place_holder)]
        CountMeasure(Box<CountMeasure>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = minute_in_hour)]
    #[holder(generate_deserialize)]
    pub struct MinuteInHour(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = month_in_year_number)]
    #[holder(generate_deserialize)]
    pub struct MonthInYearNumber(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = parameter_value)]
    #[holder(generate_deserialize)]
    pub struct ParameterValue(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PcurveOrSurface {
        #[holder(use_place_holder)]
        Pcurve(PcurveAny),
        #[holder(use_place_holder)]
        Surface(SurfaceAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonOrganizationItem {
        #[holder(use_place_holder)]
        Change(Box<Change>),
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
        #[holder(use_place_holder)]
        ConfigurationItem(Box<ConfigurationItem>),
        #[holder(use_place_holder)]
        Product(Box<Product>),
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        Contract(Box<Contract>),
        #[holder(use_place_holder)]
        SecurityClassification(Box<SecurityClassification>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonOrganizationSelect {
        #[holder(use_place_holder)]
        Person(Box<Person>),
        #[holder(use_place_holder)]
        Organization(Box<Organization>),
        #[holder(use_place_holder)]
        PersonAndOrganization(Box<PersonAndOrganization>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_measure)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleMeasure(pub f64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = positive_length_measure)]
    #[holder(generate_deserialize)]
    pub struct PositiveLengthMeasure(#[holder(use_place_holder)] pub LengthMeasure);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = positive_plane_angle_measure)]
    #[holder(generate_deserialize)]
    pub struct PositivePlaneAngleMeasure(#[holder(use_place_holder)] pub PlaneAngleMeasure);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum PreferredSurfaceCurveRepresentation {
        Curve3D,
        PcurveS1,
        PcurveS2,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ReversibleTopology {
        #[holder(use_place_holder)]
        ReversibleTopologyItem(Box<ReversibleTopologyItem>),
        #[holder(use_place_holder)]
        ListOfReversibleTopologyItem(Box<ListOfReversibleTopologyItem>),
        #[holder(use_place_holder)]
        SetOfReversibleTopologyItem(Box<SetOfReversibleTopologyItem>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ReversibleTopologyItem {
        #[holder(use_place_holder)]
        Edge(EdgeAny),
        #[holder(use_place_holder)]
        Path(PathAny),
        #[holder(use_place_holder)]
        Face(FaceAny),
        #[holder(use_place_holder)]
        FaceBound(FaceBoundAny),
        #[holder(use_place_holder)]
        ClosedShell(ClosedShellAny),
        #[holder(use_place_holder)]
        OpenShell(OpenShellAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = second_in_minute)]
    #[holder(generate_deserialize)]
    pub struct SecondInMinute(pub f64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = set_of_reversible_topology_item)]
    #[holder(generate_deserialize)]
    pub struct SetOfReversibleTopologyItem(
        #[holder(use_place_holder)] pub Vec<ReversibleTopologyItem>,
    );
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ShapeDefinition {
        #[holder(use_place_holder)]
        ProductDefinitionShape(Box<ProductDefinitionShape>),
        #[holder(use_place_holder)]
        ShapeAspect(Box<ShapeAspect>),
        #[holder(use_place_holder)]
        ShapeAspectRelationship(Box<ShapeAspectRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Shell {
        #[holder(use_place_holder)]
        VertexShell(Box<VertexShell>),
        #[holder(use_place_holder)]
        WireShell(Box<WireShell>),
        #[holder(use_place_holder)]
        OpenShell(OpenShellAny),
        #[holder(use_place_holder)]
        ClosedShell(ClosedShellAny),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum SiPrefix {
        Exa,
        Peta,
        Tera,
        Giga,
        Mega,
        Kilo,
        Hecto,
        Deca,
        Deci,
        Centi,
        Milli,
        Micro,
        Nano,
        Pico,
        Femto,
        Atto,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum SiUnitName {
        Metre,
        Gram,
        Second,
        Ampere,
        Kelvin,
        Mole,
        Candela,
        Radian,
        Steradian,
        Hertz,
        Newton,
        Pascal,
        Joule,
        Watt,
        Coulomb,
        Volt,
        Farad,
        Ohm,
        Siemens,
        Weber,
        Tesla,
        Henry,
        DegreeCelsius,
        Lumen,
        Lux,
        Becquerel,
        Gray,
        Sievert,
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = solid_angle_measure)]
    #[holder(generate_deserialize)]
    pub struct SolidAngleMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum Source {
        Made,
        Bought,
        NotKnown,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SpecifiedItem {
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        ShapeAspect(Box<ShapeAspect>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum StartRequestItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SupportedItem {
        #[holder(use_place_holder)]
        ActionDirective(Box<ActionDirective>),
        #[holder(use_place_holder)]
        Action(ActionAny),
        #[holder(use_place_holder)]
        ActionMethod(Box<ActionMethod>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceModel {
        #[holder(use_place_holder)]
        ShellBasedSurfaceModel(Box<ShellBasedSurfaceModel>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = text)]
    #[holder(generate_deserialize)]
    pub struct Text(pub String);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Transformation {
        #[holder(use_place_holder)]
        ItemDefinedTransformation(Box<ItemDefinedTransformation>),
        #[holder(use_place_holder)]
        FunctionallyDefinedTransformation(FunctionallyDefinedTransformationAny),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum TransitionCode {
        Discontinuous,
        Continuous,
        ContSameGradient,
        ContSameGradientSameCurvature,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum TrimmingPreference {
        Cartesian,
        Parameter,
        Unspecified,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TrimmingSelect {
        #[holder(use_place_holder)]
        CartesianPoint(Box<CartesianPoint>),
        #[holder(use_place_holder)]
        ParameterValue(Box<ParameterValue>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Unit {
        #[holder(use_place_holder)]
        NamedUnit(NamedUnitAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum VectorOrDirection {
        #[holder(use_place_holder)]
        Vector(Box<Vector>),
        #[holder(use_place_holder)]
        Direction(Box<Direction>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = volume_measure)]
    #[holder(generate_deserialize)]
    pub struct VolumeMeasure(pub f64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = week_in_year_number)]
    #[holder(generate_deserialize)]
    pub struct WeekInYearNumber(pub i64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum WireframeModel {
        #[holder(use_place_holder)]
        ShellBasedWireframeModel(Box<ShellBasedWireframeModel>),
        #[holder(use_place_holder)]
        EdgeBasedWireframeModel(Box<EdgeBasedWireframeModel>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum WorkItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = year_number)]
    #[holder(generate_deserialize)]
    pub struct YearNumber(pub i64);
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action)]
    #[holder(generate_deserialize)]
    pub struct Action {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub chosen_method: ActionMethod,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ActionAny {
        #[holder(use_place_holder)]
        Action(Box<Action>),
        #[holder(use_place_holder)]
        ExecutedAction(Box<ExecutedActionAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_assignment)]
    #[holder(generate_deserialize)]
    pub struct ActionAssignment {
        #[holder(use_place_holder)]
        pub assigned_action: ActionAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ActionAssignmentAny {
        #[holder(use_place_holder)]
        ActionAssignment(Box<ActionAssignment>),
        #[holder(use_place_holder)]
        Change(Box<Change>),
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_directive)]
    #[holder(generate_deserialize)]
    pub struct ActionDirective {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub analysis: Text,
        #[holder(use_place_holder)]
        pub comment: Text,
        #[holder(use_place_holder)]
        pub requests: Vec<VersionedActionRequest>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_method)]
    #[holder(generate_deserialize)]
    pub struct ActionMethod {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub consequence: Text,
        #[holder(use_place_holder)]
        pub purpose: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_assignment)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestAssignment {
        #[holder(use_place_holder)]
        pub assigned_action_request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ActionRequestAssignmentAny {
        #[holder(use_place_holder)]
        ActionRequestAssignment(Box<ActionRequestAssignment>),
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_solution)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestSolution {
        #[holder(use_place_holder)]
        pub method: ActionMethod,
        #[holder(use_place_holder)]
        pub request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_status)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestStatus {
        #[holder(use_place_holder)]
        pub status: Label,
        #[holder(use_place_holder)]
        pub assigned_request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_status)]
    #[holder(generate_deserialize)]
    pub struct ActionStatus {
        #[holder(use_place_holder)]
        pub status: Label,
        #[holder(use_place_holder)]
        pub assigned_action: ExecutedActionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = address)]
    #[holder(generate_deserialize)]
    pub struct Address {
        #[holder(use_place_holder)]
        pub internal_location: Option<Label>,
        #[holder(use_place_holder)]
        pub street_number: Option<Label>,
        #[holder(use_place_holder)]
        pub street: Option<Label>,
        #[holder(use_place_holder)]
        pub postal_box: Option<Label>,
        #[holder(use_place_holder)]
        pub town: Option<Label>,
        #[holder(use_place_holder)]
        pub region: Option<Label>,
        #[holder(use_place_holder)]
        pub postal_code: Option<Label>,
        #[holder(use_place_holder)]
        pub country: Option<Label>,
        #[holder(use_place_holder)]
        pub facsimile_number: Option<Label>,
        #[holder(use_place_holder)]
        pub telephone_number: Option<Label>,
        #[holder(use_place_holder)]
        pub electronic_mail_address: Option<Label>,
        #[holder(use_place_holder)]
        pub telex_number: Option<Label>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AddressAny {
        #[holder(use_place_holder)]
        Address(Box<Address>),
        #[holder(use_place_holder)]
        OrganizationalAddress(Box<OrganizationalAddress>),
        #[holder(use_place_holder)]
        PersonalAddress(Box<PersonalAddress>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = advanced_brep_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct AdvancedBrepShapeRepresentation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = advanced_face)]
    #[holder(generate_deserialize)]
    pub struct AdvancedFace {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub bounds: Vec<FaceBoundAny>,
        #[holder(use_place_holder)]
        pub face_geometry: SurfaceAny,
        pub same_sense: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = alternate_product_relationship)]
    #[holder(generate_deserialize)]
    pub struct AlternateProductRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub definition: Text,
        #[holder(use_place_holder)]
        pub alternate: Product,
        #[holder(use_place_holder)]
        pub base: Product,
        #[holder(use_place_holder)]
        pub basis: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_context)]
    #[holder(generate_deserialize)]
    pub struct ApplicationContext {
        #[holder(use_place_holder)]
        pub application: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_context_element)]
    #[holder(generate_deserialize)]
    pub struct ApplicationContextElement {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub frame_of_reference: ApplicationContext,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApplicationContextElementAny {
        #[holder(use_place_holder)]
        ApplicationContextElement(Box<ApplicationContextElement>),
        #[holder(use_place_holder)]
        ProductConceptContext(Box<ProductConceptContext>),
        #[holder(use_place_holder)]
        ProductContext(Box<ProductContextAny>),
        #[holder(use_place_holder)]
        ProductDefinitionContext(Box<ProductDefinitionContextAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_protocol_definition)]
    #[holder(generate_deserialize)]
    pub struct ApplicationProtocolDefinition {
        #[holder(use_place_holder)]
        pub status: Label,
        #[holder(use_place_holder)]
        pub application_interpreted_model_schema_name: Label,
        #[holder(use_place_holder)]
        pub application_protocol_year: YearNumber,
        #[holder(use_place_holder)]
        pub application: ApplicationContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval)]
    #[holder(generate_deserialize)]
    pub struct Approval {
        #[holder(use_place_holder)]
        pub status: ApprovalStatus,
        #[holder(use_place_holder)]
        pub level: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_assignment)]
    #[holder(generate_deserialize)]
    pub struct ApprovalAssignment {
        #[holder(use_place_holder)]
        pub assigned_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApprovalAssignmentAny {
        #[holder(use_place_holder)]
        ApprovalAssignment(Box<ApprovalAssignment>),
        #[holder(use_place_holder)]
        CcDesignApproval(Box<CcDesignApproval>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_date_time)]
    #[holder(generate_deserialize)]
    pub struct ApprovalDateTime {
        #[holder(use_place_holder)]
        pub date_time: DateTimeSelect,
        #[holder(use_place_holder)]
        pub dated_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_person_organization)]
    #[holder(generate_deserialize)]
    pub struct ApprovalPersonOrganization {
        #[holder(use_place_holder)]
        pub person_organization: PersonOrganizationSelect,
        #[holder(use_place_holder)]
        pub authorized_approval: Approval,
        #[holder(use_place_holder)]
        pub role: ApprovalRole,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_relationship)]
    #[holder(generate_deserialize)]
    pub struct ApprovalRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_approval: Approval,
        #[holder(use_place_holder)]
        pub related_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_role)]
    #[holder(generate_deserialize)]
    pub struct ApprovalRole {
        #[holder(use_place_holder)]
        pub role: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_status)]
    #[holder(generate_deserialize)]
    pub struct ApprovalStatus {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = area_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct AreaMeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = area_unit)]
    #[holder(generate_deserialize)]
    pub struct AreaUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = assembly_component_usage)]
    #[holder(generate_deserialize)]
    pub struct AssemblyComponentUsage {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub reference_designator: Option<Identifier>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AssemblyComponentUsageAny {
        #[holder(use_place_holder)]
        AssemblyComponentUsage(Box<AssemblyComponentUsage>),
        #[holder(use_place_holder)]
        NextAssemblyUsageOccurrence(Box<NextAssemblyUsageOccurrence>),
        #[holder(use_place_holder)]
        PromissoryUsageOccurrence(Box<PromissoryUsageOccurrence>),
        #[holder(use_place_holder)]
        QuantifiedAssemblyComponentUsage(Box<QuantifiedAssemblyComponentUsage>),
        #[holder(use_place_holder)]
        SpecifiedHigherUsageOccurrence(Box<SpecifiedHigherUsageOccurrence>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = assembly_component_usage_substitute)]
    #[holder(generate_deserialize)]
    pub struct AssemblyComponentUsageSubstitute {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub definition: Text,
        #[holder(use_place_holder)]
        pub base: AssemblyComponentUsageAny,
        #[holder(use_place_holder)]
        pub substitute: AssemblyComponentUsageAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = axis1_placement)]
    #[holder(generate_deserialize)]
    pub struct Axis1Placement {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub location: CartesianPoint,
        #[holder(use_place_holder)]
        pub axis: Option<Direction>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = axis2_placement_2d)]
    #[holder(generate_deserialize)]
    pub struct Axis2Placement2D {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub location: CartesianPoint,
        #[holder(use_place_holder)]
        pub ref_direction: Option<Direction>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = axis2_placement_3d)]
    #[holder(generate_deserialize)]
    pub struct Axis2Placement3D {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub location: CartesianPoint,
        #[holder(use_place_holder)]
        pub axis: Option<Direction>,
        #[holder(use_place_holder)]
        pub ref_direction: Option<Direction>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_curve)]
    #[holder(generate_deserialize)]
    pub struct BSplineCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        pub degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BSplineCurveAny {
        #[holder(use_place_holder)]
        BSplineCurve(Box<BSplineCurve>),
        #[holder(use_place_holder)]
        BSplineCurveWithKnots(Box<BSplineCurveWithKnots>),
        #[holder(use_place_holder)]
        BezierCurve(Box<BezierCurve>),
        #[holder(use_place_holder)]
        QuasiUniformCurve(Box<QuasiUniformCurve>),
        #[holder(use_place_holder)]
        RationalBSplineCurve(Box<RationalBSplineCurve>),
        #[holder(use_place_holder)]
        UniformCurve(Box<UniformCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_curve_with_knots)]
    #[holder(generate_deserialize)]
    pub struct BSplineCurveWithKnots {
        #[holder(use_place_holder)]
        pub name: Label,
        pub degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
        pub knot_multiplicities: Vec<i64>,
        #[holder(use_place_holder)]
        pub knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_surface)]
    #[holder(generate_deserialize)]
    pub struct BSplineSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        pub u_degree: i64,
        pub v_degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<Vec<CartesianPoint>>,
        pub surface_form: BSplineSurfaceForm,
        pub u_closed: Logical,
        pub v_closed: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BSplineSurfaceAny {
        #[holder(use_place_holder)]
        BSplineSurface(Box<BSplineSurface>),
        #[holder(use_place_holder)]
        BSplineSurfaceWithKnots(Box<BSplineSurfaceWithKnots>),
        #[holder(use_place_holder)]
        BezierSurface(Box<BezierSurface>),
        #[holder(use_place_holder)]
        QuasiUniformSurface(Box<QuasiUniformSurface>),
        #[holder(use_place_holder)]
        RationalBSplineSurface(Box<RationalBSplineSurface>),
        #[holder(use_place_holder)]
        UniformSurface(Box<UniformSurface>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_surface_with_knots)]
    #[holder(generate_deserialize)]
    pub struct BSplineSurfaceWithKnots {
        #[holder(use_place_holder)]
        pub name: Label,
        pub u_degree: i64,
        pub v_degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<Vec<CartesianPoint>>,
        pub surface_form: BSplineSurfaceForm,
        pub u_closed: Logical,
        pub v_closed: Logical,
        pub self_intersect: Logical,
        pub u_multiplicities: Vec<i64>,
        pub v_multiplicities: Vec<i64>,
        #[holder(use_place_holder)]
        pub u_knots: Vec<ParameterValue>,
        #[holder(use_place_holder)]
        pub v_knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bezier_curve)]
    #[holder(generate_deserialize)]
    pub struct BezierCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        pub degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bezier_surface)]
    #[holder(generate_deserialize)]
    pub struct BezierSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        pub u_degree: i64,
        pub v_degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<Vec<CartesianPoint>>,
        pub surface_form: BSplineSurfaceForm,
        pub u_closed: Logical,
        pub v_closed: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = boundary_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundaryCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub segments: Vec<CompositeCurveSegmentAny>,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundaryCurveAny {
        #[holder(use_place_holder)]
        BoundaryCurve(Box<BoundaryCurve>),
        #[holder(use_place_holder)]
        OuterBoundaryCurve(Box<OuterBoundaryCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundedCurve {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundedCurveAny {
        #[holder(use_place_holder)]
        BoundedCurve(Box<BoundedCurve>),
        #[holder(use_place_holder)]
        BSplineCurve(Box<BSplineCurveAny>),
        #[holder(use_place_holder)]
        BoundedPcurve(Box<BoundedPcurve>),
        #[holder(use_place_holder)]
        BoundedSurfaceCurve(Box<BoundedSurfaceCurve>),
        #[holder(use_place_holder)]
        CompositeCurve(Box<CompositeCurveAny>),
        #[holder(use_place_holder)]
        Polyline(Box<Polyline>),
        #[holder(use_place_holder)]
        TrimmedCurve(Box<TrimmedCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_pcurve)]
    #[holder(generate_deserialize)]
    pub struct BoundedPcurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub reference_to_curve: DefinitionalRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_surface)]
    #[holder(generate_deserialize)]
    pub struct BoundedSurface {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundedSurfaceAny {
        #[holder(use_place_holder)]
        BoundedSurface(Box<BoundedSurface>),
        #[holder(use_place_holder)]
        BSplineSurface(Box<BSplineSurfaceAny>),
        #[holder(use_place_holder)]
        CurveBoundedSurface(Box<CurveBoundedSurface>),
        #[holder(use_place_holder)]
        RectangularCompositeSurface(Box<RectangularCompositeSurface>),
        #[holder(use_place_holder)]
        RectangularTrimmedSurface(Box<RectangularTrimmedSurface>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_surface_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundedSurfaceCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub curve_3d: CurveAny,
        #[holder(use_place_holder)]
        pub associated_geometry: Vec<PcurveOrSurface>,
        pub master_representation: PreferredSurfaceCurveRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = brep_with_voids)]
    #[holder(generate_deserialize)]
    pub struct BrepWithVoids {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub outer: ClosedShellAny,
        #[holder(use_place_holder)]
        pub voids: Vec<OrientedClosedShell>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = calendar_date)]
    #[holder(generate_deserialize)]
    pub struct CalendarDate {
        #[holder(use_place_holder)]
        pub year_component: YearNumber,
        #[holder(use_place_holder)]
        pub day_component: DayInMonthNumber,
        #[holder(use_place_holder)]
        pub month_component: MonthInYearNumber,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_point)]
    #[holder(generate_deserialize)]
    pub struct CartesianPoint {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub coordinates: Vec<LengthMeasure>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_transformation_operator)]
    #[holder(generate_deserialize)]
    pub struct CartesianTransformationOperator {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub axis1: Option<Direction>,
        #[holder(use_place_holder)]
        pub axis2: Option<Direction>,
        #[holder(use_place_holder)]
        pub local_origin: CartesianPoint,
        pub scale: Option<f64>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CartesianTransformationOperatorAny {
        #[holder(use_place_holder)]
        CartesianTransformationOperator(Box<CartesianTransformationOperator>),
        #[holder(use_place_holder)]
        CartesianTransformationOperator3D(Box<CartesianTransformationOperator3D>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_transformation_operator_3d)]
    #[holder(generate_deserialize)]
    pub struct CartesianTransformationOperator3D {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub axis1: Option<Direction>,
        #[holder(use_place_holder)]
        pub axis2: Option<Direction>,
        #[holder(use_place_holder)]
        pub local_origin: CartesianPoint,
        pub scale: Option<f64>,
        #[holder(use_place_holder)]
        pub axis3: Option<Direction>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_approval)]
    #[holder(generate_deserialize)]
    pub struct CcDesignApproval {
        #[holder(use_place_holder)]
        pub assigned_approval: Approval,
        #[holder(use_place_holder)]
        pub items: Vec<ApprovedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_certification)]
    #[holder(generate_deserialize)]
    pub struct CcDesignCertification {
        #[holder(use_place_holder)]
        pub assigned_certification: Certification,
        #[holder(use_place_holder)]
        pub items: Vec<CertifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_contract)]
    #[holder(generate_deserialize)]
    pub struct CcDesignContract {
        #[holder(use_place_holder)]
        pub assigned_contract: Contract,
        #[holder(use_place_holder)]
        pub items: Vec<ContractedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_date_and_time_assignment)]
    #[holder(generate_deserialize)]
    pub struct CcDesignDateAndTimeAssignment {
        #[holder(use_place_holder)]
        pub assigned_date_and_time: DateAndTime,
        #[holder(use_place_holder)]
        pub role: DateTimeRole,
        #[holder(use_place_holder)]
        pub items: Vec<DateTimeItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_person_and_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct CcDesignPersonAndOrganizationAssignment {
        #[holder(use_place_holder)]
        pub assigned_person_and_organization: PersonAndOrganization,
        #[holder(use_place_holder)]
        pub role: PersonAndOrganizationRole,
        #[holder(use_place_holder)]
        pub items: Vec<PersonOrganizationItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_security_classification)]
    #[holder(generate_deserialize)]
    pub struct CcDesignSecurityClassification {
        #[holder(use_place_holder)]
        pub assigned_security_classification: SecurityClassification,
        #[holder(use_place_holder)]
        pub items: Vec<ClassifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_specification_reference)]
    #[holder(generate_deserialize)]
    pub struct CcDesignSpecificationReference {
        #[holder(use_place_holder)]
        pub assigned_document: DocumentAny,
        #[holder(use_place_holder)]
        pub source: Label,
        #[holder(use_place_holder)]
        pub items: Vec<SpecifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification)]
    #[holder(generate_deserialize)]
    pub struct Certification {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub kind: CertificationType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification_assignment)]
    #[holder(generate_deserialize)]
    pub struct CertificationAssignment {
        #[holder(use_place_holder)]
        pub assigned_certification: Certification,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CertificationAssignmentAny {
        #[holder(use_place_holder)]
        CertificationAssignment(Box<CertificationAssignment>),
        #[holder(use_place_holder)]
        CcDesignCertification(Box<CcDesignCertification>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification_type)]
    #[holder(generate_deserialize)]
    pub struct CertificationType {
        #[holder(use_place_holder)]
        pub description: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = change)]
    #[holder(generate_deserialize)]
    pub struct Change {
        #[holder(use_place_holder)]
        pub assigned_action: ActionAny,
        #[holder(use_place_holder)]
        pub items: Vec<WorkItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = change_request)]
    #[holder(generate_deserialize)]
    pub struct ChangeRequest {
        #[holder(use_place_holder)]
        pub assigned_action_request: VersionedActionRequest,
        #[holder(use_place_holder)]
        pub items: Vec<ChangeRequestItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = circle)]
    #[holder(generate_deserialize)]
    pub struct Circle {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement,
        #[holder(use_place_holder)]
        pub radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = closed_shell)]
    #[holder(generate_deserialize)]
    pub struct ClosedShell {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub cfs_faces: Vec<FaceAny>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ClosedShellAny {
        #[holder(use_place_holder)]
        ClosedShell(Box<ClosedShell>),
        #[holder(use_place_holder)]
        OrientedClosedShell(Box<OrientedClosedShell>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub segments: Vec<CompositeCurveSegmentAny>,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveAny {
        #[holder(use_place_holder)]
        CompositeCurve(Box<CompositeCurve>),
        #[holder(use_place_holder)]
        CompositeCurveOnSurface(Box<CompositeCurveOnSurfaceAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve_on_surface)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurveOnSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub segments: Vec<CompositeCurveSegmentAny>,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveOnSurfaceAny {
        #[holder(use_place_holder)]
        CompositeCurveOnSurface(Box<CompositeCurveOnSurface>),
        #[holder(use_place_holder)]
        BoundaryCurve(Box<BoundaryCurveAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve_segment)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurveSegment {
        pub transition: TransitionCode,
        pub same_sense: bool,
        #[holder(use_place_holder)]
        pub parent_curve: CurveAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveSegmentAny {
        #[holder(use_place_holder)]
        CompositeCurveSegment(Box<CompositeCurveSegment>),
        #[holder(use_place_holder)]
        ReparametrisedCompositeCurveSegment(Box<ReparametrisedCompositeCurveSegment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = configuration_design)]
    #[holder(generate_deserialize)]
    pub struct ConfigurationDesign {
        #[holder(use_place_holder)]
        pub configuration: ConfigurationItem,
        #[holder(use_place_holder)]
        pub design: ProductDefinitionFormationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = configuration_effectivity)]
    #[holder(generate_deserialize)]
    pub struct ConfigurationEffectivity {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub usage: ProductDefinitionRelationshipAny,
        #[holder(use_place_holder)]
        pub configuration: ConfigurationDesign,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = configuration_item)]
    #[holder(generate_deserialize)]
    pub struct ConfigurationItem {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Option<Text>,
        #[holder(use_place_holder)]
        pub item_concept: ProductConcept,
        #[holder(use_place_holder)]
        pub purpose: Option<Label>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = conic)]
    #[holder(generate_deserialize)]
    pub struct Conic {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ConicAny {
        #[holder(use_place_holder)]
        Conic(Box<Conic>),
        #[holder(use_place_holder)]
        Circle(Box<Circle>),
        #[holder(use_place_holder)]
        Ellipse(Box<Ellipse>),
        #[holder(use_place_holder)]
        Hyperbola(Box<Hyperbola>),
        #[holder(use_place_holder)]
        Parabola(Box<Parabola>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = conical_surface)]
    #[holder(generate_deserialize)]
    pub struct ConicalSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
        #[holder(use_place_holder)]
        pub radius: LengthMeasure,
        #[holder(use_place_holder)]
        pub semi_angle: PlaneAngleMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = connected_edge_set)]
    #[holder(generate_deserialize)]
    pub struct ConnectedEdgeSet {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub ces_edges: Vec<EdgeAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = connected_face_set)]
    #[holder(generate_deserialize)]
    pub struct ConnectedFaceSet {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub cfs_faces: Vec<FaceAny>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ConnectedFaceSetAny {
        #[holder(use_place_holder)]
        ConnectedFaceSet(Box<ConnectedFaceSet>),
        #[holder(use_place_holder)]
        ClosedShell(Box<ClosedShellAny>),
        #[holder(use_place_holder)]
        OpenShell(Box<OpenShellAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentShapeRepresentation {
        #[holder(use_place_holder)]
        pub representation_relation: ShapeRepresentationRelationship,
        #[holder(use_place_holder)]
        pub represented_product_relation: ProductDefinitionShape,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_unit)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract)]
    #[holder(generate_deserialize)]
    pub struct Contract {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub kind: ContractType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract_assignment)]
    #[holder(generate_deserialize)]
    pub struct ContractAssignment {
        #[holder(use_place_holder)]
        pub assigned_contract: Contract,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ContractAssignmentAny {
        #[holder(use_place_holder)]
        ContractAssignment(Box<ContractAssignment>),
        #[holder(use_place_holder)]
        CcDesignContract(Box<CcDesignContract>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract_type)]
    #[holder(generate_deserialize)]
    pub struct ContractType {
        #[holder(use_place_holder)]
        pub description: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = conversion_based_unit)]
    #[holder(generate_deserialize)]
    pub struct ConversionBasedUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub conversion_factor: MeasureWithUnitAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = coordinated_universal_time_offset)]
    #[holder(generate_deserialize)]
    pub struct CoordinatedUniversalTimeOffset {
        #[holder(use_place_holder)]
        pub hour_offset: HourInDay,
        #[holder(use_place_holder)]
        pub minute_offset: Option<MinuteInHour>,
        pub sense: AheadOrBehind,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve)]
    #[holder(generate_deserialize)]
    pub struct Curve {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveAny {
        #[holder(use_place_holder)]
        Curve(Box<Curve>),
        #[holder(use_place_holder)]
        BoundedCurve(Box<BoundedCurveAny>),
        #[holder(use_place_holder)]
        Conic(Box<ConicAny>),
        #[holder(use_place_holder)]
        CurveReplica(Box<CurveReplica>),
        #[holder(use_place_holder)]
        Line(Box<Line>),
        #[holder(use_place_holder)]
        OffsetCurve3D(Box<OffsetCurve3D>),
        #[holder(use_place_holder)]
        Pcurve(Box<PcurveAny>),
        #[holder(use_place_holder)]
        SurfaceCurve(Box<SurfaceCurveAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_bounded_surface)]
    #[holder(generate_deserialize)]
    pub struct CurveBoundedSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub boundaries: Vec<BoundaryCurveAny>,
        pub implicit_outer: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_replica)]
    #[holder(generate_deserialize)]
    pub struct CurveReplica {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub parent_curve: CurveAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperatorAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cylindrical_surface)]
    #[holder(generate_deserialize)]
    pub struct CylindricalSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
        #[holder(use_place_holder)]
        pub radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date)]
    #[holder(generate_deserialize)]
    pub struct Date {
        #[holder(use_place_holder)]
        pub year_component: YearNumber,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateAny {
        #[holder(use_place_holder)]
        Date(Box<Date>),
        #[holder(use_place_holder)]
        CalendarDate(Box<CalendarDate>),
        #[holder(use_place_holder)]
        OrdinalDate(Box<OrdinalDate>),
        #[holder(use_place_holder)]
        WeekOfYearAndDayDate(Box<WeekOfYearAndDayDate>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_and_time)]
    #[holder(generate_deserialize)]
    pub struct DateAndTime {
        #[holder(use_place_holder)]
        pub date_component: DateAny,
        #[holder(use_place_holder)]
        pub time_component: LocalTime,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_and_time_assignment)]
    #[holder(generate_deserialize)]
    pub struct DateAndTimeAssignment {
        #[holder(use_place_holder)]
        pub assigned_date_and_time: DateAndTime,
        #[holder(use_place_holder)]
        pub role: DateTimeRole,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateAndTimeAssignmentAny {
        #[holder(use_place_holder)]
        DateAndTimeAssignment(Box<DateAndTimeAssignment>),
        #[holder(use_place_holder)]
        CcDesignDateAndTimeAssignment(Box<CcDesignDateAndTimeAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_time_role)]
    #[holder(generate_deserialize)]
    pub struct DateTimeRole {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dated_effectivity)]
    #[holder(generate_deserialize)]
    pub struct DatedEffectivity {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub effectivity_start_date: DateAndTime,
        #[holder(use_place_holder)]
        pub effectivity_end_date: Option<DateAndTime>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = definitional_representation)]
    #[holder(generate_deserialize)]
    pub struct DefinitionalRepresentation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = degenerate_pcurve)]
    #[holder(generate_deserialize)]
    pub struct DegeneratePcurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub reference_to_curve: DefinitionalRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DegeneratePcurveAny {
        #[holder(use_place_holder)]
        DegeneratePcurve(Box<DegeneratePcurve>),
        #[holder(use_place_holder)]
        EvaluatedDegeneratePcurve(Box<EvaluatedDegeneratePcurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = degenerate_toroidal_surface)]
    #[holder(generate_deserialize)]
    pub struct DegenerateToroidalSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
        #[holder(use_place_holder)]
        pub major_radius: PositiveLengthMeasure,
        #[holder(use_place_holder)]
        pub minor_radius: PositiveLengthMeasure,
        pub select_outer: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = design_context)]
    #[holder(generate_deserialize)]
    pub struct DesignContext {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub frame_of_reference: ApplicationContext,
        #[holder(use_place_holder)]
        pub life_cycle_stage: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = design_make_from_relationship)]
    #[holder(generate_deserialize)]
    pub struct DesignMakeFromRelationship {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dimensional_exponents)]
    #[holder(generate_deserialize)]
    pub struct DimensionalExponents {
        pub length_exponent: f64,
        pub mass_exponent: f64,
        pub time_exponent: f64,
        pub electric_current_exponent: f64,
        pub thermodynamic_temperature_exponent: f64,
        pub amount_of_substance_exponent: f64,
        pub luminous_intensity_exponent: f64,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = directed_action)]
    #[holder(generate_deserialize)]
    pub struct DirectedAction {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub chosen_method: ActionMethod,
        #[holder(use_place_holder)]
        pub directive: ActionDirective,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = direction)]
    #[holder(generate_deserialize)]
    pub struct Direction {
        #[holder(use_place_holder)]
        pub name: Label,
        pub direction_ratios: Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document)]
    #[holder(generate_deserialize)]
    pub struct Document {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub kind: DocumentType,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DocumentAny {
        #[holder(use_place_holder)]
        Document(Box<Document>),
        #[holder(use_place_holder)]
        DocumentWithClass(Box<DocumentWithClass>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_reference)]
    #[holder(generate_deserialize)]
    pub struct DocumentReference {
        #[holder(use_place_holder)]
        pub assigned_document: DocumentAny,
        #[holder(use_place_holder)]
        pub source: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DocumentReferenceAny {
        #[holder(use_place_holder)]
        DocumentReference(Box<DocumentReference>),
        #[holder(use_place_holder)]
        CcDesignSpecificationReference(Box<CcDesignSpecificationReference>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_relationship)]
    #[holder(generate_deserialize)]
    pub struct DocumentRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_document: DocumentAny,
        #[holder(use_place_holder)]
        pub related_document: DocumentAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_type)]
    #[holder(generate_deserialize)]
    pub struct DocumentType {
        #[holder(use_place_holder)]
        pub product_data_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_usage_constraint)]
    #[holder(generate_deserialize)]
    pub struct DocumentUsageConstraint {
        #[holder(use_place_holder)]
        pub source: DocumentAny,
        #[holder(use_place_holder)]
        pub subject_element: Label,
        #[holder(use_place_holder)]
        pub subject_element_value: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_with_class)]
    #[holder(generate_deserialize)]
    pub struct DocumentWithClass {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub kind: DocumentType,
        #[holder(use_place_holder)]
        pub class: Identifier,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge)]
    #[holder(generate_deserialize)]
    pub struct Edge {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub edge_start: VertexAny,
        #[holder(use_place_holder)]
        pub edge_end: VertexAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum EdgeAny {
        #[holder(use_place_holder)]
        Edge(Box<Edge>),
        #[holder(use_place_holder)]
        EdgeCurve(Box<EdgeCurve>),
        #[holder(use_place_holder)]
        OrientedEdge(Box<OrientedEdge>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_based_wireframe_model)]
    #[holder(generate_deserialize)]
    pub struct EdgeBasedWireframeModel {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub ebwm_boundary: Vec<ConnectedEdgeSet>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_based_wireframe_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct EdgeBasedWireframeShapeRepresentation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_curve)]
    #[holder(generate_deserialize)]
    pub struct EdgeCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub edge_start: VertexAny,
        #[holder(use_place_holder)]
        pub edge_end: VertexAny,
        #[holder(use_place_holder)]
        pub edge_geometry: CurveAny,
        pub same_sense: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_loop)]
    #[holder(generate_deserialize)]
    pub struct EdgeLoop {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub edge_list: Vec<OrientedEdge>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = effectivity)]
    #[holder(generate_deserialize)]
    pub struct Effectivity {
        #[holder(use_place_holder)]
        pub id: Identifier,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum EffectivityAny {
        #[holder(use_place_holder)]
        Effectivity(Box<Effectivity>),
        #[holder(use_place_holder)]
        DatedEffectivity(Box<DatedEffectivity>),
        #[holder(use_place_holder)]
        LotEffectivity(Box<LotEffectivity>),
        #[holder(use_place_holder)]
        ProductDefinitionEffectivity(Box<ProductDefinitionEffectivityAny>),
        #[holder(use_place_holder)]
        SerialNumberedEffectivity(Box<SerialNumberedEffectivity>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = elementary_surface)]
    #[holder(generate_deserialize)]
    pub struct ElementarySurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ElementarySurfaceAny {
        #[holder(use_place_holder)]
        ElementarySurface(Box<ElementarySurface>),
        #[holder(use_place_holder)]
        ConicalSurface(Box<ConicalSurface>),
        #[holder(use_place_holder)]
        CylindricalSurface(Box<CylindricalSurface>),
        #[holder(use_place_holder)]
        Plane(Box<Plane>),
        #[holder(use_place_holder)]
        SphericalSurface(Box<SphericalSurface>),
        #[holder(use_place_holder)]
        ToroidalSurface(Box<ToroidalSurfaceAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = ellipse)]
    #[holder(generate_deserialize)]
    pub struct Ellipse {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement,
        #[holder(use_place_holder)]
        pub semi_axis_1: PositiveLengthMeasure,
        #[holder(use_place_holder)]
        pub semi_axis_2: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = evaluated_degenerate_pcurve)]
    #[holder(generate_deserialize)]
    pub struct EvaluatedDegeneratePcurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub reference_to_curve: DefinitionalRepresentation,
        #[holder(use_place_holder)]
        pub equivalent_point: CartesianPoint,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = executed_action)]
    #[holder(generate_deserialize)]
    pub struct ExecutedAction {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub chosen_method: ActionMethod,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ExecutedActionAny {
        #[holder(use_place_holder)]
        ExecutedAction(Box<ExecutedAction>),
        #[holder(use_place_holder)]
        DirectedAction(Box<DirectedAction>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = face)]
    #[holder(generate_deserialize)]
    pub struct Face {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub bounds: Vec<FaceBoundAny>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceAny {
        #[holder(use_place_holder)]
        Face(Box<Face>),
        #[holder(use_place_holder)]
        FaceSurface(Box<FaceSurfaceAny>),
        #[holder(use_place_holder)]
        OrientedFace(Box<OrientedFace>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = face_bound)]
    #[holder(generate_deserialize)]
    pub struct FaceBound {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub bound: LoopAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceBoundAny {
        #[holder(use_place_holder)]
        FaceBound(Box<FaceBound>),
        #[holder(use_place_holder)]
        FaceOuterBound(Box<FaceOuterBound>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = face_outer_bound)]
    #[holder(generate_deserialize)]
    pub struct FaceOuterBound {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub bound: LoopAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = face_surface)]
    #[holder(generate_deserialize)]
    pub struct FaceSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub bounds: Vec<FaceBoundAny>,
        #[holder(use_place_holder)]
        pub face_geometry: SurfaceAny,
        pub same_sense: bool,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceSurfaceAny {
        #[holder(use_place_holder)]
        FaceSurface(Box<FaceSurface>),
        #[holder(use_place_holder)]
        AdvancedFace(Box<AdvancedFace>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = faceted_brep)]
    #[holder(generate_deserialize)]
    pub struct FacetedBrep {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub outer: ClosedShellAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = faceted_brep_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct FacetedBrepShapeRepresentation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = founded_item)]
    #[holder(generate_deserialize)]
    pub struct FoundedItem {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FoundedItemAny {
        #[holder(use_place_holder)]
        FoundedItem(Box<FoundedItem>),
        #[holder(use_place_holder)]
        CompositeCurveSegment(Box<CompositeCurveSegmentAny>),
        #[holder(use_place_holder)]
        SurfacePatch(Box<SurfacePatch>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = functionally_defined_transformation)]
    #[holder(generate_deserialize)]
    pub struct FunctionallyDefinedTransformation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FunctionallyDefinedTransformationAny {
        #[holder(use_place_holder)]
        FunctionallyDefinedTransformation(Box<FunctionallyDefinedTransformation>),
        #[holder(use_place_holder)]
        CartesianTransformationOperator(Box<CartesianTransformationOperatorAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_curve_set)]
    #[holder(generate_deserialize)]
    pub struct GeometricCurveSet {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub elements: Vec<GeometricSetSelect>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_representation_context)]
    #[holder(generate_deserialize)]
    pub struct GeometricRepresentationContext {
        #[holder(use_place_holder)]
        pub context_identifier: Identifier,
        #[holder(use_place_holder)]
        pub context_type: Text,
        #[holder(use_place_holder)]
        pub coordinate_space_dimension: DimensionCount,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_representation_item)]
    #[holder(generate_deserialize)]
    pub struct GeometricRepresentationItem {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricRepresentationItemAny {
        #[holder(use_place_holder)]
        GeometricRepresentationItem(Box<GeometricRepresentationItem>),
        #[holder(use_place_holder)]
        CartesianTransformationOperator(Box<CartesianTransformationOperatorAny>),
        #[holder(use_place_holder)]
        Curve(Box<CurveAny>),
        #[holder(use_place_holder)]
        Direction(Box<Direction>),
        #[holder(use_place_holder)]
        EdgeBasedWireframeModel(Box<EdgeBasedWireframeModel>),
        #[holder(use_place_holder)]
        EdgeCurve(Box<EdgeCurve>),
        #[holder(use_place_holder)]
        FaceSurface(Box<FaceSurfaceAny>),
        #[holder(use_place_holder)]
        GeometricSet(Box<GeometricSetAny>),
        #[holder(use_place_holder)]
        Placement(Box<PlacementAny>),
        #[holder(use_place_holder)]
        Point(Box<PointAny>),
        #[holder(use_place_holder)]
        PolyLoop(Box<PolyLoop>),
        #[holder(use_place_holder)]
        ShellBasedSurfaceModel(Box<ShellBasedSurfaceModel>),
        #[holder(use_place_holder)]
        ShellBasedWireframeModel(Box<ShellBasedWireframeModel>),
        #[holder(use_place_holder)]
        SolidModel(Box<SolidModelAny>),
        #[holder(use_place_holder)]
        Surface(Box<SurfaceAny>),
        #[holder(use_place_holder)]
        Vector(Box<Vector>),
        #[holder(use_place_holder)]
        VertexPoint(Box<VertexPoint>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_set)]
    #[holder(generate_deserialize)]
    pub struct GeometricSet {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub elements: Vec<GeometricSetSelect>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricSetAny {
        #[holder(use_place_holder)]
        GeometricSet(Box<GeometricSet>),
        #[holder(use_place_holder)]
        GeometricCurveSet(Box<GeometricCurveSet>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometrically_bounded_surface_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct GeometricallyBoundedSurfaceShapeRepresentation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometrically_bounded_wireframe_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct GeometricallyBoundedWireframeShapeRepresentation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = global_uncertainty_assigned_context)]
    #[holder(generate_deserialize)]
    pub struct GlobalUncertaintyAssignedContext {
        #[holder(use_place_holder)]
        pub context_identifier: Identifier,
        #[holder(use_place_holder)]
        pub context_type: Text,
        #[holder(use_place_holder)]
        pub uncertainty: Vec<UncertaintyMeasureWithUnit>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = global_unit_assigned_context)]
    #[holder(generate_deserialize)]
    pub struct GlobalUnitAssignedContext {
        #[holder(use_place_holder)]
        pub context_identifier: Identifier,
        #[holder(use_place_holder)]
        pub context_type: Text,
        #[holder(use_place_holder)]
        pub units: Vec<Unit>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = hyperbola)]
    #[holder(generate_deserialize)]
    pub struct Hyperbola {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement,
        #[holder(use_place_holder)]
        pub semi_axis: PositiveLengthMeasure,
        #[holder(use_place_holder)]
        pub semi_imag_axis: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = intersection_curve)]
    #[holder(generate_deserialize)]
    pub struct IntersectionCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub curve_3d: CurveAny,
        #[holder(use_place_holder)]
        pub associated_geometry: Vec<PcurveOrSurface>,
        pub master_representation: PreferredSurfaceCurveRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = item_defined_transformation)]
    #[holder(generate_deserialize)]
    pub struct ItemDefinedTransformation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub transform_item_1: RepresentationItemAny,
        #[holder(use_place_holder)]
        pub transform_item_2: RepresentationItemAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = length_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct LengthMeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = length_unit)]
    #[holder(generate_deserialize)]
    pub struct LengthUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = line)]
    #[holder(generate_deserialize)]
    pub struct Line {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub pnt: CartesianPoint,
        #[holder(use_place_holder)]
        pub dir: Vector,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = local_time)]
    #[holder(generate_deserialize)]
    pub struct LocalTime {
        #[holder(use_place_holder)]
        pub hour_component: HourInDay,
        #[holder(use_place_holder)]
        pub minute_component: Option<MinuteInHour>,
        #[holder(use_place_holder)]
        pub second_component: Option<SecondInMinute>,
        #[holder(use_place_holder)]
        pub zone: CoordinatedUniversalTimeOffset,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = r#loop)]
    #[holder(generate_deserialize)]
    pub struct Loop {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum LoopAny {
        #[holder(use_place_holder)]
        Loop(Box<Loop>),
        #[holder(use_place_holder)]
        EdgeLoop(Box<EdgeLoop>),
        #[holder(use_place_holder)]
        PolyLoop(Box<PolyLoop>),
        #[holder(use_place_holder)]
        VertexLoop(Box<VertexLoop>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = lot_effectivity)]
    #[holder(generate_deserialize)]
    pub struct LotEffectivity {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub effectivity_lot_id: Identifier,
        #[holder(use_place_holder)]
        pub effectivity_lot_size: MeasureWithUnitAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = manifold_solid_brep)]
    #[holder(generate_deserialize)]
    pub struct ManifoldSolidBrep {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub outer: ClosedShellAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ManifoldSolidBrepAny {
        #[holder(use_place_holder)]
        ManifoldSolidBrep(Box<ManifoldSolidBrep>),
        #[holder(use_place_holder)]
        BrepWithVoids(Box<BrepWithVoids>),
        #[holder(use_place_holder)]
        FacetedBrep(Box<FacetedBrep>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = manifold_surface_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ManifoldSurfaceShapeRepresentation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = mapped_item)]
    #[holder(generate_deserialize)]
    pub struct MappedItem {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub mapping_source: RepresentationMap,
        #[holder(use_place_holder)]
        pub mapping_target: RepresentationItemAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = mass_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct MassMeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = mass_unit)]
    #[holder(generate_deserialize)]
    pub struct MassUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct MeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MeasureWithUnitAny {
        #[holder(use_place_holder)]
        MeasureWithUnit(Box<MeasureWithUnit>),
        #[holder(use_place_holder)]
        AreaMeasureWithUnit(Box<AreaMeasureWithUnit>),
        #[holder(use_place_holder)]
        LengthMeasureWithUnit(Box<LengthMeasureWithUnit>),
        #[holder(use_place_holder)]
        MassMeasureWithUnit(Box<MassMeasureWithUnit>),
        #[holder(use_place_holder)]
        PlaneAngleMeasureWithUnit(Box<PlaneAngleMeasureWithUnit>),
        #[holder(use_place_holder)]
        SolidAngleMeasureWithUnit(Box<SolidAngleMeasureWithUnit>),
        #[holder(use_place_holder)]
        UncertaintyMeasureWithUnit(Box<UncertaintyMeasureWithUnit>),
        #[holder(use_place_holder)]
        VolumeMeasureWithUnit(Box<VolumeMeasureWithUnit>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = mechanical_context)]
    #[holder(generate_deserialize)]
    pub struct MechanicalContext {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub frame_of_reference: ApplicationContext,
        #[holder(use_place_holder)]
        pub discipline_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = named_unit)]
    #[holder(generate_deserialize)]
    pub struct NamedUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum NamedUnitAny {
        #[holder(use_place_holder)]
        NamedUnit(Box<NamedUnit>),
        #[holder(use_place_holder)]
        AreaUnit(Box<AreaUnit>),
        #[holder(use_place_holder)]
        ContextDependentUnit(Box<ContextDependentUnit>),
        #[holder(use_place_holder)]
        ConversionBasedUnit(Box<ConversionBasedUnit>),
        #[holder(use_place_holder)]
        LengthUnit(Box<LengthUnit>),
        #[holder(use_place_holder)]
        MassUnit(Box<MassUnit>),
        #[holder(use_place_holder)]
        PlaneAngleUnit(Box<PlaneAngleUnit>),
        #[holder(use_place_holder)]
        SiUnit(Box<SiUnit>),
        #[holder(use_place_holder)]
        SolidAngleUnit(Box<SolidAngleUnit>),
        #[holder(use_place_holder)]
        VolumeUnit(Box<VolumeUnit>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = next_assembly_usage_occurrence)]
    #[holder(generate_deserialize)]
    pub struct NextAssemblyUsageOccurrence {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub reference_designator: Option<Identifier>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = offset_curve_3d)]
    #[holder(generate_deserialize)]
    pub struct OffsetCurve3D {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        #[holder(use_place_holder)]
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
        #[holder(use_place_holder)]
        pub ref_direction: Direction,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = offset_surface)]
    #[holder(generate_deserialize)]
    pub struct OffsetSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = open_shell)]
    #[holder(generate_deserialize)]
    pub struct OpenShell {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub cfs_faces: Vec<FaceAny>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum OpenShellAny {
        #[holder(use_place_holder)]
        OpenShell(Box<OpenShell>),
        #[holder(use_place_holder)]
        OrientedOpenShell(Box<OrientedOpenShell>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = ordinal_date)]
    #[holder(generate_deserialize)]
    pub struct OrdinalDate {
        #[holder(use_place_holder)]
        pub year_component: YearNumber,
        #[holder(use_place_holder)]
        pub day_component: DayInYearNumber,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization)]
    #[holder(generate_deserialize)]
    pub struct Organization {
        #[holder(use_place_holder)]
        pub id: Option<Identifier>,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization_relationship)]
    #[holder(generate_deserialize)]
    pub struct OrganizationRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_organization: Organization,
        #[holder(use_place_holder)]
        pub related_organization: Organization,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organizational_address)]
    #[holder(generate_deserialize)]
    pub struct OrganizationalAddress {
        #[holder(use_place_holder)]
        pub internal_location: Option<Label>,
        #[holder(use_place_holder)]
        pub street_number: Option<Label>,
        #[holder(use_place_holder)]
        pub street: Option<Label>,
        #[holder(use_place_holder)]
        pub postal_box: Option<Label>,
        #[holder(use_place_holder)]
        pub town: Option<Label>,
        #[holder(use_place_holder)]
        pub region: Option<Label>,
        #[holder(use_place_holder)]
        pub postal_code: Option<Label>,
        #[holder(use_place_holder)]
        pub country: Option<Label>,
        #[holder(use_place_holder)]
        pub facsimile_number: Option<Label>,
        #[holder(use_place_holder)]
        pub telephone_number: Option<Label>,
        #[holder(use_place_holder)]
        pub electronic_mail_address: Option<Label>,
        #[holder(use_place_holder)]
        pub telex_number: Option<Label>,
        #[holder(use_place_holder)]
        pub organizations: Vec<Organization>,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organizational_project)]
    #[holder(generate_deserialize)]
    pub struct OrganizationalProject {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub responsible_organizations: Vec<Organization>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_closed_shell)]
    #[holder(generate_deserialize)]
    pub struct OrientedClosedShell {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub cfs_faces: Vec<FaceAny>,
        #[holder(use_place_holder)]
        pub closed_shell_element: ClosedShellAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_edge)]
    #[holder(generate_deserialize)]
    pub struct OrientedEdge {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub edge_start: VertexAny,
        #[holder(use_place_holder)]
        pub edge_end: VertexAny,
        #[holder(use_place_holder)]
        pub edge_element: EdgeAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_face)]
    #[holder(generate_deserialize)]
    pub struct OrientedFace {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub bounds: Vec<FaceBoundAny>,
        #[holder(use_place_holder)]
        pub face_element: FaceAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_open_shell)]
    #[holder(generate_deserialize)]
    pub struct OrientedOpenShell {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub cfs_faces: Vec<FaceAny>,
        #[holder(use_place_holder)]
        pub open_shell_element: OpenShellAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_path)]
    #[holder(generate_deserialize)]
    pub struct OrientedPath {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub edge_list: Vec<OrientedEdge>,
        #[holder(use_place_holder)]
        pub path_element: PathAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = outer_boundary_curve)]
    #[holder(generate_deserialize)]
    pub struct OuterBoundaryCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub segments: Vec<CompositeCurveSegmentAny>,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = parabola)]
    #[holder(generate_deserialize)]
    pub struct Parabola {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement,
        #[holder(use_place_holder)]
        pub focal_dist: LengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = parametric_representation_context)]
    #[holder(generate_deserialize)]
    pub struct ParametricRepresentationContext {
        #[holder(use_place_holder)]
        pub context_identifier: Identifier,
        #[holder(use_place_holder)]
        pub context_type: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = path)]
    #[holder(generate_deserialize)]
    pub struct Path {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub edge_list: Vec<OrientedEdge>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PathAny {
        #[holder(use_place_holder)]
        Path(Box<Path>),
        #[holder(use_place_holder)]
        EdgeLoop(Box<EdgeLoop>),
        #[holder(use_place_holder)]
        OrientedPath(Box<OrientedPath>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pcurve)]
    #[holder(generate_deserialize)]
    pub struct Pcurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub reference_to_curve: DefinitionalRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PcurveAny {
        #[holder(use_place_holder)]
        Pcurve(Box<Pcurve>),
        #[holder(use_place_holder)]
        BoundedPcurve(Box<BoundedPcurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person)]
    #[holder(generate_deserialize)]
    pub struct Person {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub last_name: Option<Label>,
        #[holder(use_place_holder)]
        pub first_name: Option<Label>,
        #[holder(use_place_holder)]
        pub middle_names: Option<Vec<Label>>,
        #[holder(use_place_holder)]
        pub prefix_titles: Option<Vec<Label>>,
        #[holder(use_place_holder)]
        pub suffix_titles: Option<Vec<Label>>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganization {
        #[holder(use_place_holder)]
        pub the_person: Person,
        #[holder(use_place_holder)]
        pub the_organization: Organization,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganizationAssignment {
        #[holder(use_place_holder)]
        pub assigned_person_and_organization: PersonAndOrganization,
        #[holder(use_place_holder)]
        pub role: PersonAndOrganizationRole,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonAndOrganizationAssignmentAny {
        #[holder(use_place_holder)]
        PersonAndOrganizationAssignment(Box<PersonAndOrganizationAssignment>),
        #[holder(use_place_holder)]
        CcDesignPersonAndOrganizationAssignment(Box<CcDesignPersonAndOrganizationAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization_role)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganizationRole {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = personal_address)]
    #[holder(generate_deserialize)]
    pub struct PersonalAddress {
        #[holder(use_place_holder)]
        pub internal_location: Option<Label>,
        #[holder(use_place_holder)]
        pub street_number: Option<Label>,
        #[holder(use_place_holder)]
        pub street: Option<Label>,
        #[holder(use_place_holder)]
        pub postal_box: Option<Label>,
        #[holder(use_place_holder)]
        pub town: Option<Label>,
        #[holder(use_place_holder)]
        pub region: Option<Label>,
        #[holder(use_place_holder)]
        pub postal_code: Option<Label>,
        #[holder(use_place_holder)]
        pub country: Option<Label>,
        #[holder(use_place_holder)]
        pub facsimile_number: Option<Label>,
        #[holder(use_place_holder)]
        pub telephone_number: Option<Label>,
        #[holder(use_place_holder)]
        pub electronic_mail_address: Option<Label>,
        #[holder(use_place_holder)]
        pub telex_number: Option<Label>,
        #[holder(use_place_holder)]
        pub people: Vec<Person>,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = placement)]
    #[holder(generate_deserialize)]
    pub struct Placement {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub location: CartesianPoint,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PlacementAny {
        #[holder(use_place_holder)]
        Placement(Box<Placement>),
        #[holder(use_place_holder)]
        Axis1Placement(Box<Axis1Placement>),
        #[holder(use_place_holder)]
        Axis2Placement2D(Box<Axis2Placement2D>),
        #[holder(use_place_holder)]
        Axis2Placement3D(Box<Axis2Placement3D>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane)]
    #[holder(generate_deserialize)]
    pub struct Plane {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleMeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_unit)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = point)]
    #[holder(generate_deserialize)]
    pub struct Point {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PointAny {
        #[holder(use_place_holder)]
        Point(Box<Point>),
        #[holder(use_place_holder)]
        CartesianPoint(Box<CartesianPoint>),
        #[holder(use_place_holder)]
        DegeneratePcurve(Box<DegeneratePcurveAny>),
        #[holder(use_place_holder)]
        PointOnCurve(Box<PointOnCurve>),
        #[holder(use_place_holder)]
        PointOnSurface(Box<PointOnSurface>),
        #[holder(use_place_holder)]
        PointReplica(Box<PointReplica>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = point_on_curve)]
    #[holder(generate_deserialize)]
    pub struct PointOnCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        #[holder(use_place_holder)]
        pub point_parameter: ParameterValue,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = point_on_surface)]
    #[holder(generate_deserialize)]
    pub struct PointOnSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub point_parameter_u: ParameterValue,
        #[holder(use_place_holder)]
        pub point_parameter_v: ParameterValue,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = point_replica)]
    #[holder(generate_deserialize)]
    pub struct PointReplica {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub parent_pt: PointAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperatorAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = poly_loop)]
    #[holder(generate_deserialize)]
    pub struct PolyLoop {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub polygon: Vec<CartesianPoint>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = polyline)]
    #[holder(generate_deserialize)]
    pub struct Polyline {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub points: Vec<CartesianPoint>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product)]
    #[holder(generate_deserialize)]
    pub struct Product {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub frame_of_reference: Vec<ProductContextAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_category)]
    #[holder(generate_deserialize)]
    pub struct ProductCategory {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Option<Text>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductCategoryAny {
        #[holder(use_place_holder)]
        ProductCategory(Box<ProductCategory>),
        #[holder(use_place_holder)]
        ProductRelatedProductCategory(Box<ProductRelatedProductCategory>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_category_relationship)]
    #[holder(generate_deserialize)]
    pub struct ProductCategoryRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub category: ProductCategoryAny,
        #[holder(use_place_holder)]
        pub sub_category: ProductCategoryAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_concept)]
    #[holder(generate_deserialize)]
    pub struct ProductConcept {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub market_context: ProductConceptContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_concept_context)]
    #[holder(generate_deserialize)]
    pub struct ProductConceptContext {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub frame_of_reference: ApplicationContext,
        #[holder(use_place_holder)]
        pub market_segment_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_context)]
    #[holder(generate_deserialize)]
    pub struct ProductContext {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub frame_of_reference: ApplicationContext,
        #[holder(use_place_holder)]
        pub discipline_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductContextAny {
        #[holder(use_place_holder)]
        ProductContext(Box<ProductContext>),
        #[holder(use_place_holder)]
        MechanicalContext(Box<MechanicalContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinition {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub formation: ProductDefinitionFormationAny,
        #[holder(use_place_holder)]
        pub frame_of_reference: ProductDefinitionContextAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionAny {
        #[holder(use_place_holder)]
        ProductDefinition(Box<ProductDefinition>),
        #[holder(use_place_holder)]
        ProductDefinitionWithAssociatedDocuments(Box<ProductDefinitionWithAssociatedDocuments>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_context)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionContext {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub frame_of_reference: ApplicationContext,
        #[holder(use_place_holder)]
        pub life_cycle_stage: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionContextAny {
        #[holder(use_place_holder)]
        ProductDefinitionContext(Box<ProductDefinitionContext>),
        #[holder(use_place_holder)]
        DesignContext(Box<DesignContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_effectivity)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionEffectivity {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub usage: ProductDefinitionRelationshipAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionEffectivityAny {
        #[holder(use_place_holder)]
        ProductDefinitionEffectivity(Box<ProductDefinitionEffectivity>),
        #[holder(use_place_holder)]
        ConfigurationEffectivity(Box<ConfigurationEffectivity>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_formation)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionFormation {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub of_product: Product,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionFormationAny {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
        #[holder(use_place_holder)]
        ProductDefinitionFormationWithSpecifiedSource(
            Box<ProductDefinitionFormationWithSpecifiedSource>,
        ),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_formation_with_specified_source)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionFormationWithSpecifiedSource {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub of_product: Product,
        pub make_or_buy: Source,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_relationship)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionRelationship {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionRelationshipAny {
        #[holder(use_place_holder)]
        ProductDefinitionRelationship(Box<ProductDefinitionRelationship>),
        #[holder(use_place_holder)]
        DesignMakeFromRelationship(Box<DesignMakeFromRelationship>),
        #[holder(use_place_holder)]
        ProductDefinitionUsage(Box<ProductDefinitionUsageAny>),
        #[holder(use_place_holder)]
        SuppliedPartRelationship(Box<SuppliedPartRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_shape)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionShape {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub definition: CharacterizedDefinition,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_usage)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionUsage {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionUsageAny {
        #[holder(use_place_holder)]
        ProductDefinitionUsage(Box<ProductDefinitionUsage>),
        #[holder(use_place_holder)]
        AssemblyComponentUsage(Box<AssemblyComponentUsageAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_with_associated_documents)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionWithAssociatedDocuments {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub formation: ProductDefinitionFormationAny,
        #[holder(use_place_holder)]
        pub frame_of_reference: ProductDefinitionContextAny,
        #[holder(use_place_holder)]
        pub documentation_ids: Vec<DocumentAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_related_product_category)]
    #[holder(generate_deserialize)]
    pub struct ProductRelatedProductCategory {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Option<Text>,
        #[holder(use_place_holder)]
        pub products: Vec<Product>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = promissory_usage_occurrence)]
    #[holder(generate_deserialize)]
    pub struct PromissoryUsageOccurrence {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub reference_designator: Option<Identifier>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = property_definition)]
    #[holder(generate_deserialize)]
    pub struct PropertyDefinition {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub definition: CharacterizedDefinition,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PropertyDefinitionAny {
        #[holder(use_place_holder)]
        PropertyDefinition(Box<PropertyDefinition>),
        #[holder(use_place_holder)]
        ProductDefinitionShape(Box<ProductDefinitionShape>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = property_definition_representation)]
    #[holder(generate_deserialize)]
    pub struct PropertyDefinitionRepresentation {
        #[holder(use_place_holder)]
        pub definition: PropertyDefinitionAny,
        #[holder(use_place_holder)]
        pub used_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PropertyDefinitionRepresentationAny {
        #[holder(use_place_holder)]
        PropertyDefinitionRepresentation(Box<PropertyDefinitionRepresentation>),
        #[holder(use_place_holder)]
        ShapeDefinitionRepresentation(Box<ShapeDefinitionRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = quantified_assembly_component_usage)]
    #[holder(generate_deserialize)]
    pub struct QuantifiedAssemblyComponentUsage {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub reference_designator: Option<Identifier>,
        #[holder(use_place_holder)]
        pub quantity: MeasureWithUnitAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = quasi_uniform_curve)]
    #[holder(generate_deserialize)]
    pub struct QuasiUniformCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        pub degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = quasi_uniform_surface)]
    #[holder(generate_deserialize)]
    pub struct QuasiUniformSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        pub u_degree: i64,
        pub v_degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<Vec<CartesianPoint>>,
        pub surface_form: BSplineSurfaceForm,
        pub u_closed: Logical,
        pub v_closed: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = rational_b_spline_curve)]
    #[holder(generate_deserialize)]
    pub struct RationalBSplineCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        pub degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
        pub weights_data: Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = rational_b_spline_surface)]
    #[holder(generate_deserialize)]
    pub struct RationalBSplineSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        pub u_degree: i64,
        pub v_degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<Vec<CartesianPoint>>,
        pub surface_form: BSplineSurfaceForm,
        pub u_closed: Logical,
        pub v_closed: Logical,
        pub self_intersect: Logical,
        pub weights_data: Vec<Vec<f64>>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = rectangular_composite_surface)]
    #[holder(generate_deserialize)]
    pub struct RectangularCompositeSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub segments: Vec<Vec<SurfacePatch>>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = rectangular_trimmed_surface)]
    #[holder(generate_deserialize)]
    pub struct RectangularTrimmedSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub u1: ParameterValue,
        #[holder(use_place_holder)]
        pub u2: ParameterValue,
        #[holder(use_place_holder)]
        pub v1: ParameterValue,
        #[holder(use_place_holder)]
        pub v2: ParameterValue,
        pub usense: bool,
        pub vsense: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = reparametrised_composite_curve_segment)]
    #[holder(generate_deserialize)]
    pub struct ReparametrisedCompositeCurveSegment {
        pub transition: TransitionCode,
        pub same_sense: bool,
        #[holder(use_place_holder)]
        pub parent_curve: CurveAny,
        #[holder(use_place_holder)]
        pub param_length: ParameterValue,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation)]
    #[holder(generate_deserialize)]
    pub struct Representation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationAny {
        #[holder(use_place_holder)]
        Representation(Box<Representation>),
        #[holder(use_place_holder)]
        DefinitionalRepresentation(Box<DefinitionalRepresentation>),
        #[holder(use_place_holder)]
        ShapeRepresentation(Box<ShapeRepresentationAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_context)]
    #[holder(generate_deserialize)]
    pub struct RepresentationContext {
        #[holder(use_place_holder)]
        pub context_identifier: Identifier,
        #[holder(use_place_holder)]
        pub context_type: Text,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationContextAny {
        #[holder(use_place_holder)]
        RepresentationContext(Box<RepresentationContext>),
        #[holder(use_place_holder)]
        GeometricRepresentationContext(Box<GeometricRepresentationContext>),
        #[holder(use_place_holder)]
        GlobalUncertaintyAssignedContext(Box<GlobalUncertaintyAssignedContext>),
        #[holder(use_place_holder)]
        GlobalUnitAssignedContext(Box<GlobalUnitAssignedContext>),
        #[holder(use_place_holder)]
        ParametricRepresentationContext(Box<ParametricRepresentationContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_item)]
    #[holder(generate_deserialize)]
    pub struct RepresentationItem {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationItemAny {
        #[holder(use_place_holder)]
        RepresentationItem(Box<RepresentationItem>),
        #[holder(use_place_holder)]
        GeometricRepresentationItem(Box<GeometricRepresentationItemAny>),
        #[holder(use_place_holder)]
        MappedItem(Box<MappedItem>),
        #[holder(use_place_holder)]
        TopologicalRepresentationItem(Box<TopologicalRepresentationItemAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_map)]
    #[holder(generate_deserialize)]
    pub struct RepresentationMap {
        #[holder(use_place_holder)]
        pub mapping_origin: RepresentationItemAny,
        #[holder(use_place_holder)]
        pub mapped_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_relationship)]
    #[holder(generate_deserialize)]
    pub struct RepresentationRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub rep_1: RepresentationAny,
        #[holder(use_place_holder)]
        pub rep_2: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationRelationshipAny {
        #[holder(use_place_holder)]
        RepresentationRelationship(Box<RepresentationRelationship>),
        #[holder(use_place_holder)]
        RepresentationRelationshipWithTransformation(
            Box<RepresentationRelationshipWithTransformation>,
        ),
        #[holder(use_place_holder)]
        ShapeRepresentationRelationship(Box<ShapeRepresentationRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_relationship_with_transformation)]
    #[holder(generate_deserialize)]
    pub struct RepresentationRelationshipWithTransformation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub rep_1: RepresentationAny,
        #[holder(use_place_holder)]
        pub rep_2: RepresentationAny,
        #[holder(use_place_holder)]
        pub transformation_operator: Transformation,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = seam_curve)]
    #[holder(generate_deserialize)]
    pub struct SeamCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub curve_3d: CurveAny,
        #[holder(use_place_holder)]
        pub associated_geometry: Vec<PcurveOrSurface>,
        pub master_representation: PreferredSurfaceCurveRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassification {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub security_level: SecurityClassificationLevel,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification_assignment)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassificationAssignment {
        #[holder(use_place_holder)]
        pub assigned_security_classification: SecurityClassification,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SecurityClassificationAssignmentAny {
        #[holder(use_place_holder)]
        SecurityClassificationAssignment(Box<SecurityClassificationAssignment>),
        #[holder(use_place_holder)]
        CcDesignSecurityClassification(Box<CcDesignSecurityClassification>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification_level)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassificationLevel {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = serial_numbered_effectivity)]
    #[holder(generate_deserialize)]
    pub struct SerialNumberedEffectivity {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub effectivity_start_id: Identifier,
        #[holder(use_place_holder)]
        pub effectivity_end_id: Option<Identifier>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_aspect)]
    #[holder(generate_deserialize)]
    pub struct ShapeAspect {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub of_shape: ProductDefinitionShape,
        pub product_definitional: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_aspect_relationship)]
    #[holder(generate_deserialize)]
    pub struct ShapeAspectRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_shape_aspect: ShapeAspect,
        #[holder(use_place_holder)]
        pub related_shape_aspect: ShapeAspect,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_definition_representation)]
    #[holder(generate_deserialize)]
    pub struct ShapeDefinitionRepresentation {
        #[holder(use_place_holder)]
        pub definition: PropertyDefinitionAny,
        #[holder(use_place_holder)]
        pub used_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ShapeRepresentation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ShapeRepresentationAny {
        #[holder(use_place_holder)]
        ShapeRepresentation(Box<ShapeRepresentation>),
        #[holder(use_place_holder)]
        AdvancedBrepShapeRepresentation(Box<AdvancedBrepShapeRepresentation>),
        #[holder(use_place_holder)]
        EdgeBasedWireframeShapeRepresentation(Box<EdgeBasedWireframeShapeRepresentation>),
        #[holder(use_place_holder)]
        FacetedBrepShapeRepresentation(Box<FacetedBrepShapeRepresentation>),
        #[holder(use_place_holder)]
        GeometricallyBoundedSurfaceShapeRepresentation(
            Box<GeometricallyBoundedSurfaceShapeRepresentation>,
        ),
        #[holder(use_place_holder)]
        GeometricallyBoundedWireframeShapeRepresentation(
            Box<GeometricallyBoundedWireframeShapeRepresentation>,
        ),
        #[holder(use_place_holder)]
        ManifoldSurfaceShapeRepresentation(Box<ManifoldSurfaceShapeRepresentation>),
        #[holder(use_place_holder)]
        ShellBasedWireframeShapeRepresentation(Box<ShellBasedWireframeShapeRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_representation_relationship)]
    #[holder(generate_deserialize)]
    pub struct ShapeRepresentationRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub rep_1: RepresentationAny,
        #[holder(use_place_holder)]
        pub rep_2: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shell_based_surface_model)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedSurfaceModel {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub sbsm_boundary: Vec<Shell>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shell_based_wireframe_model)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedWireframeModel {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub sbwm_boundary: Vec<Shell>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shell_based_wireframe_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedWireframeShapeRepresentation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = si_unit)]
    #[holder(generate_deserialize)]
    pub struct SiUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = solid_angle_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct SolidAngleMeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = solid_angle_unit)]
    #[holder(generate_deserialize)]
    pub struct SolidAngleUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = solid_model)]
    #[holder(generate_deserialize)]
    pub struct SolidModel {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SolidModelAny {
        #[holder(use_place_holder)]
        SolidModel(Box<SolidModel>),
        #[holder(use_place_holder)]
        ManifoldSolidBrep(Box<ManifoldSolidBrepAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = specified_higher_usage_occurrence)]
    #[holder(generate_deserialize)]
    pub struct SpecifiedHigherUsageOccurrence {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub reference_designator: Option<Identifier>,
        #[holder(use_place_holder)]
        pub upper_usage: AssemblyComponentUsageAny,
        #[holder(use_place_holder)]
        pub next_usage: NextAssemblyUsageOccurrence,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = spherical_surface)]
    #[holder(generate_deserialize)]
    pub struct SphericalSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
        #[holder(use_place_holder)]
        pub radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = start_request)]
    #[holder(generate_deserialize)]
    pub struct StartRequest {
        #[holder(use_place_holder)]
        pub assigned_action_request: VersionedActionRequest,
        #[holder(use_place_holder)]
        pub items: Vec<StartRequestItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = start_work)]
    #[holder(generate_deserialize)]
    pub struct StartWork {
        #[holder(use_place_holder)]
        pub assigned_action: ActionAny,
        #[holder(use_place_holder)]
        pub items: Vec<WorkItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = supplied_part_relationship)]
    #[holder(generate_deserialize)]
    pub struct SuppliedPartRelationship {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface)]
    #[holder(generate_deserialize)]
    pub struct Surface {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceAny {
        #[holder(use_place_holder)]
        Surface(Box<Surface>),
        #[holder(use_place_holder)]
        BoundedSurface(Box<BoundedSurfaceAny>),
        #[holder(use_place_holder)]
        ElementarySurface(Box<ElementarySurfaceAny>),
        #[holder(use_place_holder)]
        OffsetSurface(Box<OffsetSurface>),
        #[holder(use_place_holder)]
        SurfaceReplica(Box<SurfaceReplica>),
        #[holder(use_place_holder)]
        SweptSurface(Box<SweptSurfaceAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_curve)]
    #[holder(generate_deserialize)]
    pub struct SurfaceCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub curve_3d: CurveAny,
        #[holder(use_place_holder)]
        pub associated_geometry: Vec<PcurveOrSurface>,
        pub master_representation: PreferredSurfaceCurveRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceCurveAny {
        #[holder(use_place_holder)]
        SurfaceCurve(Box<SurfaceCurve>),
        #[holder(use_place_holder)]
        BoundedSurfaceCurve(Box<BoundedSurfaceCurve>),
        #[holder(use_place_holder)]
        IntersectionCurve(Box<IntersectionCurve>),
        #[holder(use_place_holder)]
        SeamCurve(Box<SeamCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_of_linear_extrusion)]
    #[holder(generate_deserialize)]
    pub struct SurfaceOfLinearExtrusion {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub swept_curve: CurveAny,
        #[holder(use_place_holder)]
        pub extrusion_axis: Vector,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_of_revolution)]
    #[holder(generate_deserialize)]
    pub struct SurfaceOfRevolution {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub swept_curve: CurveAny,
        #[holder(use_place_holder)]
        pub axis_position: Axis1Placement,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_patch)]
    #[holder(generate_deserialize)]
    pub struct SurfacePatch {
        #[holder(use_place_holder)]
        pub parent_surface: BoundedSurfaceAny,
        pub u_transition: TransitionCode,
        pub v_transition: TransitionCode,
        pub u_sense: bool,
        pub v_sense: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_replica)]
    #[holder(generate_deserialize)]
    pub struct SurfaceReplica {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub parent_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperator3D,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = swept_surface)]
    #[holder(generate_deserialize)]
    pub struct SweptSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub swept_curve: CurveAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SweptSurfaceAny {
        #[holder(use_place_holder)]
        SweptSurface(Box<SweptSurface>),
        #[holder(use_place_holder)]
        SurfaceOfLinearExtrusion(Box<SurfaceOfLinearExtrusion>),
        #[holder(use_place_holder)]
        SurfaceOfRevolution(Box<SurfaceOfRevolution>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = topological_representation_item)]
    #[holder(generate_deserialize)]
    pub struct TopologicalRepresentationItem {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TopologicalRepresentationItemAny {
        #[holder(use_place_holder)]
        TopologicalRepresentationItem(Box<TopologicalRepresentationItem>),
        #[holder(use_place_holder)]
        ConnectedEdgeSet(Box<ConnectedEdgeSet>),
        #[holder(use_place_holder)]
        ConnectedFaceSet(Box<ConnectedFaceSetAny>),
        #[holder(use_place_holder)]
        Edge(Box<EdgeAny>),
        #[holder(use_place_holder)]
        Face(Box<FaceAny>),
        #[holder(use_place_holder)]
        FaceBound(Box<FaceBoundAny>),
        #[holder(use_place_holder)]
        Loop(Box<LoopAny>),
        #[holder(use_place_holder)]
        Path(Box<PathAny>),
        #[holder(use_place_holder)]
        Vertex(Box<VertexAny>),
        #[holder(use_place_holder)]
        VertexShell(Box<VertexShell>),
        #[holder(use_place_holder)]
        WireShell(Box<WireShell>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = toroidal_surface)]
    #[holder(generate_deserialize)]
    pub struct ToroidalSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
        #[holder(use_place_holder)]
        pub major_radius: PositiveLengthMeasure,
        #[holder(use_place_holder)]
        pub minor_radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ToroidalSurfaceAny {
        #[holder(use_place_holder)]
        ToroidalSurface(Box<ToroidalSurface>),
        #[holder(use_place_holder)]
        DegenerateToroidalSurface(Box<DegenerateToroidalSurface>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = trimmed_curve)]
    #[holder(generate_deserialize)]
    pub struct TrimmedCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        #[holder(use_place_holder)]
        pub trim_1: Vec<TrimmingSelect>,
        #[holder(use_place_holder)]
        pub trim_2: Vec<TrimmingSelect>,
        pub sense_agreement: bool,
        pub master_representation: TrimmingPreference,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = uncertainty_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct UncertaintyMeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = uniform_curve)]
    #[holder(generate_deserialize)]
    pub struct UniformCurve {
        #[holder(use_place_holder)]
        pub name: Label,
        pub degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = uniform_surface)]
    #[holder(generate_deserialize)]
    pub struct UniformSurface {
        #[holder(use_place_holder)]
        pub name: Label,
        pub u_degree: i64,
        pub v_degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<Vec<CartesianPoint>>,
        pub surface_form: BSplineSurfaceForm,
        pub u_closed: Logical,
        pub v_closed: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vector)]
    #[holder(generate_deserialize)]
    pub struct Vector {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub orientation: Direction,
        #[holder(use_place_holder)]
        pub magnitude: LengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = versioned_action_request)]
    #[holder(generate_deserialize)]
    pub struct VersionedActionRequest {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub version: Label,
        #[holder(use_place_holder)]
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex)]
    #[holder(generate_deserialize)]
    pub struct Vertex {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum VertexAny {
        #[holder(use_place_holder)]
        Vertex(Box<Vertex>),
        #[holder(use_place_holder)]
        VertexPoint(Box<VertexPoint>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex_loop)]
    #[holder(generate_deserialize)]
    pub struct VertexLoop {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub loop_vertex: VertexAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex_point)]
    #[holder(generate_deserialize)]
    pub struct VertexPoint {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub vertex_geometry: PointAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex_shell)]
    #[holder(generate_deserialize)]
    pub struct VertexShell {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub vertex_shell_extent: VertexLoop,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = volume_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct VolumeMeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = volume_unit)]
    #[holder(generate_deserialize)]
    pub struct VolumeUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = week_of_year_and_day_date)]
    #[holder(generate_deserialize)]
    pub struct WeekOfYearAndDayDate {
        #[holder(use_place_holder)]
        pub year_component: YearNumber,
        #[holder(use_place_holder)]
        pub week_component: WeekInYearNumber,
        #[holder(use_place_holder)]
        pub day_component: Option<DayInWeekNumber>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = wire_shell)]
    #[holder(generate_deserialize)]
    pub struct WireShell {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub wire_shell_extent: Vec<LoopAny>,
    }
}
