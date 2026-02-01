//React
import React from "react";

//Material UI Components
import {
	Box,
	Button,
	Card,
	CardActionArea,
	CardMedia,
	createTheme,
	Dialog,
	DialogActions,
	DialogContent,
	DialogContentText,
	DialogTitle,
	FormControl,
	Grid,
	IconButton,
	InputLabel,
	Link,
	MenuItem,
	Select,
	Tooltip,
	Typography,
	useMediaQuery,
} from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";
import { QRCodeSVG } from "qrcode.react";
import { enqueueSnackbar } from "notistack";

export default function HomePage() {
	const [udid, setUDID] = React.useState("");
	const [devices, setDevices] = React.useState({});
	const [devModeDialogOpen, setDevModeDialogOpen] = React.useState(false);

	const prefersDarkMode = useMediaQuery("(prefers-color-scheme: dark)");
	const darkMode = useMediaQuery("(prefers-color-scheme: dark)")
		? "dark"
		: "light";
	const theme = createTheme({
		palette: {
			mode: darkMode,
			primary: {
				main: "#ffa7ac",
				contrastText: "#fff",
			},
			secondary: {
				main: "#ff8185ff",
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
		text: {
			position: "absolute",
			color: "#fff",
			fontWeight: "bold",
			fontSize: "3em",
		},
	};

	let ShortcutQRCode = (
		<QRCodeSVG
			size={100}
			value="https://www.icloud.com/shortcuts/602aa49d3d6f4938846fabd4684fe9a8"
		/>
	);

	let invoke = window.__TAURI__.core.invoke;
	async function getDevices() {
		return await invoke("get_devices");
	}
	window.getDevices = getDevices;
	window.generatePairingFile = async (udid, filePath) => {
		const debug = await invoke("generate_pairing_file", {
			udid,
			destination: filePath,
		});
		return debug;
	};

	async function setupDevice(udid) {
		const devModeEnabled = await invoke("get_device_in_dev_mode", { udid });
		if (!devModeEnabled) {
			await invoke("reveal_dev_mode", { udid });
			setDevModeDialogOpen(true);
			return;
		}

		const debug = await invoke("setup_device", { udid });
		return debug;
	}

	const fetchDevices = async () => {
		const devices = await window.getDevices();
		setDevices(devices);
	};

	React.useEffect(() => {
		fetchDevices();
	}, []);

	return (
		<div>
			<Dialog
				open={devModeDialogOpen}
				onClose={() => setDevModeDialogOpen(false)}
				aria-labelledby="alert-dialog-title"
				aria-describedby="alert-dialog-description"
			>
				<DialogTitle id="alert-dialog-title">{"Developer mode"}</DialogTitle>
				<DialogContent>
					<DialogContentText id="alert-dialog-description">
						Auto Capture relies on developer mode to your iOS device to allow
						screen capture functionality. You do not appear to have developer
						mode enabled on your iOS device. Please enable developer mode in the
						Settings app under Privacy & Security &gt; Developer Mode. After
						enabling, developer mode, your device will restart and prompt you
						with additional confirmation dialogs. Once complete, please
						re-attempt the device setup process.
					</DialogContentText>
				</DialogContent>
				<DialogActions>
					<Button onClick={() => setDevModeDialogOpen(false)} autoFocus>
						Ok
					</Button>
				</DialogActions>
			</Dialog>
			<Grid
				container
				spacing={0}
				alignItems="center"
				justifyContent="center"
				direction="column"
			>
				<Grid
					container
					direction="row"
					justifyContent="center"
					alignItems="center"
				></Grid>
				<Typography
					variant="h5"
					sx={{
						marginTop: theme.spacing(2),
					}}
				>
					Instructions
				</Typography>
				<Typography
					sx={{
						marginRight: 2,
					}}
				>
					<ol>
						<li>
							Install the Auto Capture app on your iOS device from the App
							Store.
						</li>
						{/*
							// if on windows show additional step for installing iTunes
							navigator.userAgent.indexOf("Windows") !== -1 ? (
								<li>
									Ensure you have iTunes installed on your computer.{" "}
									<b>Important: </b> The Windows Store version of iTunes does
									not include the necessary drivers. Please download iTunes
									directly from Apple's website:{" "}
									<Link
										href="https://www.apple.com/itunes/download/win64"
										target="_blank"
										rel="noopener"
									>
										https://www.apple.com/itunes/download/win64
									</Link>
								</li>
							) : null
							*/}
						{/*
						<li>
							Scan the following QR code to add the "Capture Device" shortcut to
							your iOS device:
							<br />
							{ShortcutQRCode}
							<br />
							This shortcut will allow you to easily trigger a screen capture
							using the side button (Requires iPhone 15 Pro or later, or any
							iPhone 16-series device or later.) or back tap using the guide on
							the Auto Capture website.
						</li>*/}
						<li>Connect your iOS device to your computer via USB.</li>
						<li>Unlock your iOS device and go to the home screen.</li>
						<li>
							If a message appears asking if you would like to trust this
							computer, press trust.
						</li>
						<li>
							Click the "Refresh Devices" button below to detect your connected
							device.
						</li>
						<li>Select your device from the dropdown menu.</li>
						<li>
							Click the "Setup Device" button to copy the necessary information
							to your device.
						</li>
						<li>On your iOS device, tap trust this computer when prompted.</li>
						<li>
							You can unplug your device after you receive a setup complete
							message on the pairing app. Make sure to restart Auto Capture. You
							can now continue the guide on the{" "}
							<Link
								href="https://halfeatentoast.com/#/AutoCapture/Guides"
								target="_blank"
								rel="noopener"
							>
								Auto Capture website
							</Link>
							.
						</li>
					</ol>
				</Typography>

				<Typography
					variant="body2"
					sx={{
						marginBottom: theme.spacing(2),
						marginX: theme.spacing(3),
					}}
				>
					<strong>Note:</strong> If your device does not appear in the dropdown,
					try disconnecting and reconnecting it, ensuring it is unlocked and on
					the home screen.
				</Typography>

				<Grid
					container
					direction="row"
					justifyContent="center"
					alignItems="center"
				>
					<Grid item>
						<FormControl fullWidth>
							<InputLabel id="demo-simple-select-label">Device</InputLabel>
							<Select
								labelId="demo-simple-select-label"
								id="demo-simple-select"
								sx={{
									minWidth: 384,
								}}
								value={udid}
								label="Device"
								onChange={(event) => setUDID(event.target.value)}
							>
								{Object.keys(devices).map((key) => (
									<MenuItem value={devices[key]}>{key}</MenuItem>
								))}
							</Select>
						</FormControl>
					</Grid>
					<Grid
						item
						sx={{
							marginLeft: theme.spacing(2),
						}}
					>
						<Tooltip title="Refresh Devices">
							<IconButton aria-label="refresh devices" onClick={fetchDevices}>
								<RefreshIcon />
							</IconButton>
						</Tooltip>
					</Grid>
				</Grid>
				<Button
					variant="contained"
					color="primary"
					sx={{
						marginTop: theme.spacing(2),
						marginBottom: theme.spacing(1),
					}}
					onClick={async () => {
						try {
							await setupDevice(udid);
							enqueueSnackbar("Device setup complete!", {
								variant: "success",
							});
						} catch (e) {
							// notistack error notification
							console.error("Error setting up device:", e);
							enqueueSnackbar("Failed to setup device: " + e.message, {
								variant: "error",
							});
						}
					}}
				>
					Setup Device
				</Button>
			</Grid>
		</div>
	);
}
