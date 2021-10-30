const HEADER = "Title";

const NAVBAR_DATA = [
	{ id: 1, url: "/",				label: "Home" },
	{ id: 2, url: "#",		label: "" },
];

const BANNER_DATA = {
	HEADING: "",
	DESCRIPTION: ""
};

const SERVICE_DATA = {
	HEADING: "Our Services",
	ALL_SERVICES: "All Services",
	SERVICE_LIST: [
		{
			LABEL: "",
			DESCRIPTION: "",
			URL: "images/products/photo1.png"
		},
		{
			LABEL: "",
			DESCRIPTION: "",
			URL: "images/products/photo2.png"
		},
		{
			LABEL: "",
			DESCRIPTION: "",
			URL: "images/products/photo3.png"
		}
	]
};

const ABOUT_DATA = {
	HEADING: "",
	TITLE: "",
	IMAGE_URL: "",
	WHY_CHOOSE_US_LIST: [
		"",
		"",
		"",
		""
	]
};

const SOCIAL_DATA = {
	HEADING: "",
	SOCIAL_LIST: [
		{
			label: "Facebook",
			url: "https://www.facebook.com/mypsamty",
			image: "images/social/instagram-icon.png"
		},
		{
			label: "Instagram",
			url: "https://www.instagram.com/mypsaplasticosmty",
			image: "images/social/facebook-icon.png"
		}
	]
};

const FOOTER_DATA = {
	HEADING: "Contact us",
	CONTACT_DETAILS: [
		{
			DESCRIPTION: "MYPSA MTY",
			ADDRESS: "Av. Lázaro Cárdenas 265, \
				Cumbres 1º Sector, 64390 Monterrey, N.L.",
			MOBILE: ["+52 81 8348 4182", "+52 81 8348 0077"],
			EMAIL: ["marco.ferrer@mypsaplasticos.com", 
				"ventas@mypsaplasticos.com"]
		},
		{
			DESCRIPTION: "MYPSA MÉXICO (BOTELLAS PET)",
			ADDRESS: "Carretera México Cuautitlán km. \
				31.5 Col. Loma Bonita en Cuautitlán Edo. de México",
			MOBILE: ["+52 55 5870 2588", "+52 55 5870 3516"],
			EMAIL: ["ventas@regiopet.com"]
		},
		{
			DESCRIPTION: "REGIO PET",
			ADDRESS: "Ave. Las Palmas #111 Santa Catarina N.L. \
				s/colonia, Carretera Monterrey-Saltillo cruz \
				con Villa de García",
			MOBILE: "+52 81 8073 6122 al 29",
			EMAIL: ["ventas@regiopet.com"]
		}


	],
	SUBSCRIBE_NEWSLETTER: "Subscribe newsletter",
	SUBSCRIBE: "Subscribe"
};

const MOCK_DATA = {
	HEADER,
	NAVBAR_DATA,
	BANNER_DATA,
	SERVICE_DATA,
	ABOUT_DATA,
	TESTIMONIAL_DATA,
	SOCIAL_DATA,
	FOOTER_DATA
};

export default MOCK_DATA;
