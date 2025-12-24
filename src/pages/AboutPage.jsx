//React
import React from "react";

//Material UI Components
import {
	createTheme,
	Grid,
	Link,
	Typography,
	useMediaQuery,
} from "@mui/material";
import { cyan } from "@mui/material/colors";

export default function AboutPage() {
	const prefersDarkMode = useMediaQuery("(prefers-color-scheme: dark)");

	const theme = createTheme({
		palette: {
			mode: prefersDarkMode ? "dark" : "light",
			primary: {
				main: cyan[500],
				contrastText: "#fff",
			},
			secondary: {
				main: cyan["A400"],
			},
		},
	});

	const classes = {
		button: {
			margin: theme.spacing(1),
		},
		main: {
			padding: theme.spacing(3),
		},
		background: {
			backgroundImage: "/img/background.jpeg",
			backgroundPosition: "center",
			backgroundSize: "cover",
			backgroundRepeat: "no-repeat",
			filter: "blur(8px)",
			height: "100vh",
			display: "flex",
			alignItems: "center",
			justifyContent: "center",
		},
		text: {
			position: "absolute",
			color: "#fff",
			fontWeight: "bold",
			fontSize: "3em",
		},
	};

	return (
		<Grid
			container
			spacing={0}
			alignItems="center"
			justifyContent="center"
			direction="column"
			sx={{
				padding: theme.spacing(2),
				paddingTop: theme.spacing(4),
			}}
		>
			<Typography
				sx={{
					position: "relative",
				}}
				variant="h3"
			>
				About
			</Typography>

			<Typography>
				Auto Capture Pair is a utility to setup and pair devices for the Auto
				Capture app.
			</Typography>
			{/*Licensed libraries (include link)*/}
			<Typography sx={{ marginTop: theme.spacing(4) }} variant="h6">
				This project uses the following libraries:
			</Typography>
			<ul>
				<li>
					<Link href="https://mui.com/" target="_blank" rel="noopener">
						Material UI
					</Link>{" "}
					- MIT License
				</li>
				<li>
					<Link href="https://notistack.com/" target="_blank" rel="noopener">
						notistack
					</Link>{" "}
					- MIT License
				</li>
				<li>
					<Link
						href="https://github.com/jkcoxson/idevice"
						target="_blank"
						rel="noopener"
					>
						idevice
					</Link>{" "}
					- MIT License
				</li>
				<li>
					<Link
						href="https://github.com/jkcoxson/idevice_pair"
						target="_blank"
						rel="noopener"
					>
						idevice_pair
					</Link>{" "}
					- MIT License
				</li>
			</ul>
		</Grid>
	);
}
