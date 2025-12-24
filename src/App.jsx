import "./App.css";
//import HomePage from "./pages/HomePage";
//import UploadPage from "./pages/UploadPage";
//import DownloadPage from "./pages/DownloadPage";
//import AdminDownloadPage from "./pages/AdminDownloadPage";
import HomePage from "./pages/HomePage";
import AboutPage from "./pages/AboutPage";
//import SettingsPage from "./pages/SettingsPage";

//React
import React from "react";

//Material UI Components
import {
	AppBar,
	Box,
	createTheme,
	CssBaseline,
	Menu,
	ThemeProvider,
	StyledEngineProvider,
	Toolbar,
	Typography,
	useMediaQuery,
	IconButton,
	Button,
	MenuItem,
	SvgIcon,
} from "@mui/material";

//Material UI Icons
import HomeIcon from "@mui/icons-material/Home";
import MinimizeIcon from "@mui/icons-material/Minimize";
import CheckBoxOutlineBlankIcon from "@mui/icons-material/CheckBoxOutlineBlank";
import CloseIcon from "@mui/icons-material/Close";
import CameraIcon from "@mui/icons-material/Camera";
import AutoCapIcon from "./autoCap Icon.svg?react";

//React Router
import { HashRouter, Routes, Route, Link } from "react-router-dom";
import { grey, orange, teal } from "@mui/material/colors";

function App() {
	const [aboutMenuOpen, setAboutMenuOpen] = React.useState(false);

	const darkMode = useMediaQuery("(prefers-color-scheme: dark)")
		? "dark"
		: "light";

	const theme = createTheme({
		palette: {
			mode: darkMode,
			primary: {
				main: "#d38286ff",
				contrastText: "#fff",
			},
			secondary: {
				main: "#ff8185ff",
			},
		},
	});

	// Set drawer width to be the size of the icons
	const drawerWidth = theme.spacing(7) + 1;

	const classes = {
		root: {
			display: "flex",
			flexDirection: "column",
			minHeight: "100vh",
		},
		drawerPaper: {
			width: drawerWidth,
		},
		content: {
			flexGrow: 1,
			padding: theme.spacing(3),
			marginLeft: drawerWidth,
		},
	};

	return (
		<div style={classes.root}>
			<StyledEngineProvider injectFirst>
				<ThemeProvider theme={theme}>
					<CssBaseline />
					<HashRouter>
						<AppBar position="fixed" enableColorOnDark elevation={16}>
							<Toolbar variant="dense">
								<SvgIcon
									size="large"
									color="inherit"
									aria-label="menu"
									sx={{ mr: 2 }}
								>
									<AutoCapIcon width="24px" height="24px" />
								</SvgIcon>

								<Typography
									variant="h6"
									noWrap
									component="a"
									href="#/"
									sx={{
										color: "white",
										textDecoration: "none",
									}}
								>
									Auto Capture Pair{" "}
								</Typography>
								<Box sx={{ flexGrow: 1 }} />
								<Button sx={{ color: "white" }} component={Link} to="/">
									Pair
								</Button>
								<Button
									sx={{ color: "white", display: "block" }}
									component={Link}
									to="/About"
								>
									About
								</Button>
								{/*<Button
									component={Link}
									to="/Settings"
									sx={{ my: 2, color: "white", display: "block" }}
								>
									Settings
								</Button>*/}
							</Toolbar>
						</AppBar>
						<Toolbar variant="dense" />

						<main>
							<Routes>
								<Route exact path="/" element={<HomePage />} />
								<Route exact path="/About" element={<AboutPage />} />
								{/*<Route exact path="/Settings" element={<SettingsPage />} />*/}
							</Routes>
						</main>
						<div style={{ flexGrow: 1 }} />
						<Box
							sx={{
								width: "100%",
								textAlign: "center",
								padding: theme.spacing(1),
								backgroundColor: grey[900],
								color: "white",
							}}
						>
							<Typography variant="body2">
								&copy; 2025 Half Eaten Toast LLC. Support:
								support@halfeatentoast.com
							</Typography>
						</Box>
					</HashRouter>
				</ThemeProvider>
			</StyledEngineProvider>
		</div>
	);
}

export default App;
