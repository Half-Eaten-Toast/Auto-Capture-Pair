//React
import React from "react";

//Material UI Components
import {
	Switch,
	Divider,
	FormControlLabel,
	Grid,
	IconButton,
	TextField,
	Typography,
	InputAdornment,
	Select,
	MenuItem,
	FormControl,
	InputLabel,
} from "@mui/material";

//Material UI Icons
import ContentPasteIcon from "@mui/icons-material/ContentPaste";
import SaveIcon from "@mui/icons-material/Save";
import FolderOpenIcon from "@mui/icons-material/FolderOpen";

export default function SettingsPage() {
	// Download
	const [outputLocation, setOutputLocation] = React.useState();

	// Admin Download
	const [downloadToken, setDownloadToken] = React.useState();
	const [downloadMemSize, setDownloadMemSize] = React.useState();

	// Upload
	const [showUpload, setShowUpload] = React.useState();
	const [hyperionToken, setHyperionToken] = React.useState();
	const [uploadToken, setUploadToken] = React.useState();
	const [serverID, setServerID] = React.useState();
	const [categoryID, setCategoryID] = React.useState();
	const [downloadEmbedChannelID, setDownloadEmbedChannelID] = React.useState();
	const [compressionAlgorithm, setCompressionAlgorithm] = React.useState();
	const [compressionLevel, setCompressionLevel] = React.useState();
	const [uploadChunkSize, setUploadChunkSize] = React.useState();

	// Shared
	const [noProxy, setNoProxy] = React.useState();

	const classes = {
		root: {
			display: "flex",
		},
	};

	return (
		<div>
			<Grid
				container
				spacing={0}
				alignItems="center"
				justifyContent="center"
				direction="column"
			>
				<form
					className={classes.root}
					noValidate
					autoComplete="off"
					onSubmit={(event) => {
						event.preventDefault();
						window.setSettings();
					}}
				>
					<Grid
						container
						spacing={0}
						alignItems="center"
						justifyContent="center"
						direction="column"
					>
						<Typography variant="h2" component="h2">
							Download
						</Typography>

						<Grid
							container
							spacing={0}
							alignItems="center"
							justifyContent="center"
							direction="row"
						>
							<TextField
								variant="outlined"
								style={{
									width: "500px",
									margin: "10px",
								}}
								id="outputLocation"
								label="Default Download Output Location"
								onChange={(event) => {
									setOutputLocation(event.target.value);
								}}
								value={outputLocation}
							/>
							<TextField
								id="downloadMemSizeInput"
								label="Default Download Memory Allocation Per Task"
								variant="outlined"
								style={{
									width: "500px",
									margin: "10px",
								}}
								type="number"
								InputLabelProps={{
									shrink: true,
								}}
								InputProps={{
									endAdornment: (
										<InputAdornment position="end">MiB</InputAdornment>
									),
								}}
								value={downloadMemSize}
								onChange={(event) => {
									setDownloadMemSize(event.target.value);
								}}
							/>
						</Grid>

						<Divider
							style={{
								width: "100%",
							}}
						/>

						<Typography variant="h2" component="h2">
							Upload
						</Typography>

						<FormControlLabel
							control={
								<Switch
									onChange={(event) => {
										setShowUpload(event.target.checked);
									}}
									checked={showUpload}
									name="showUploadSwitch"
									id="showUploadSwitch"
								/>
							}
							label="Show Uploads"
						/>
						<TextField
							id="hyperionTokenInput"
							label="Hyperion Token"
							variant="outlined"
							style={{
								width: "500px",
								margin: "10px",
							}}
							disabled={!showUpload}
							onChange={(event) => {
								setHyperionToken(event.target.value);
							}}
							value={hyperionToken}
							InputProps={{
								endAdornment: (
									<IconButton
										aria-label="paste"
										onClick={() => {
											navigator.clipboard.readText().then((text) => {
												setHyperionToken(text);
											});
										}}
										edge="end"
										disabled={!showUpload}
									>
										<ContentPasteIcon />
									</IconButton>
								),
							}}
						/>
						{/*
                        <TextField
                            id="uploadTokenInput"
                            label="Upload Token"
                            variant="outlined"
                            style={{
                                width: "500px",
                                margin: "10px",
                            }}
                            disabled={!showUpload}
                            onChange={(event) => {
                                setUploadToken(event.target.value);
                            }}
                            value={uploadToken}
                            InputProps={{
                                endAdornment: (
                                    <IconButton
                                        aria-label="paste"
                                        onClick={() => {
                                            navigator.clipboard.readText().then((text) => {
                                                setUploadToken(text);
                                            });
                                        }}
                                        edge="end"
                                        disabled={!showUpload}
                                    >
                                        <ContentPasteIcon />
                                    </IconButton>
                                ),
                            }}
                        />
                        <TextField
                            id="serverIDInput"
                            label="Upload Server ID"
                            variant="outlined"
                            style={{
                                width: "500px",
                                margin: "10px",
                            }}
                            disabled={!showUpload}
                            onChange={(event) => {
                                setServerID(event.target.value);
                            }}
                            value={serverID}
                            InputProps={{
                                endAdornment: (
                                    <IconButton
                                        aria-label="paste"
                                        onClick={() => {
                                            navigator.clipboard.readText().then((text) => {
                                                setServerID(text);
                                            });
                                        }}
                                        edge="end"
                                        disabled={!showUpload}
                                    >
                                        <ContentPasteIcon />
                                    </IconButton>
                                ),
                            }}
                        />
                        <TextField
                            id="categoryIDInput"
                            label="Default Upload Category ID"
                            variant="outlined"
                            style={{
                                width: "500px",
                                margin: "10px",
                            }}
                            disabled={!showUpload}
                            onChange={(event) => {
                                setCategoryID(event.target.value);
                            }}
                            value={categoryID}
                            InputProps={{
                                endAdornment: (
                                    <IconButton
                                        aria-label="paste"
                                        onClick={() => {
                                            navigator.clipboard.readText().then((text) => {
                                                setCategoryID(text);
                                            });
                                        }}
                                        edge="end"
                                        disabled={!showUpload}
                                    >
                                        <ContentPasteIcon />
                                    </IconButton>
                                ),
                            }}
                        />
                        <TextField
                            id="downloadEmbedChannelIDInput"
                            label="Download Embed Channel ID"
                            variant="outlined"
                            style={{
                                width: "500px",
                                margin: "10px",
                            }}
                            disabled={!showUpload}
                            onChange={(event) => {
                                setDownloadEmbedChannelID(event.target.value);
                            }}
                            value={downloadEmbedChannelID}
                            InputProps={{
                                endAdornment: (
                                    <IconButton
                                        aria-label="paste"
                                        onClick={() => {
                                            navigator.clipboard.readText().then((text) => {
                                                setDownloadEmbedChannelID(text);
                                            });
                                        }}
                                        edge="end"
                                        disabled={!showUpload}
                                    >
                                        <ContentPasteIcon />
                                    </IconButton>
                                ),
                            }}
                        />*/}
						<FormControl>
							<InputLabel variant="outlined" htmlFor="compressionAlgorithm">
								Compression Algorithm
							</InputLabel>
							<Select
								id="compressionAlgorithm"
								label="Compression Algorithm"
								variant="outlined"
								style={{
									width: "500px",
									margin: "10px",
								}}
								disabled={!showUpload}
								value={compressionAlgorithm}
								onChange={(event) => {
									setCompressionAlgorithm(event.target.value);
								}}
							>
								<MenuItem value={"none"}>None</MenuItem>
								<MenuItem value={"gzip"}>gzip</MenuItem>
								<MenuItem value={"deflate"}>Deflate</MenuItem>
								{/*<MenuItem value={"brotli"}>Brotli</MenuItem> slow af*/}
							</Select>
						</FormControl>
						<TextField
							id="compressionLevelInput"
							label="Compression Level"
							variant="outlined"
							style={{
								width: "500px",
								margin: "10px",
							}}
							disabled={!showUpload}
							type="number"
							InputLabelProps={{
								shrink: true,
							}}
							value={compressionLevel}
							onChange={(event) => {
								setCompressionLevel(event.target.value);
							}}
						/>
						<TextField
							id="uploadChunkSizeInput"
							label="Default Upload Chunk Size (in MiB)"
							variant="outlined"
							style={{
								width: "500px",
								margin: "10px",
							}}
							disabled={!showUpload}
							type="number"
							InputLabelProps={{
								shrink: true,
							}}
							InputProps={{
								endAdornment: (
									<InputAdornment position="end">MiB</InputAdornment>
								),
								inputProps: { min: 0, max: 25 },
							}}
							value={uploadChunkSize}
							onChange={(event) => {
								setUploadChunkSize(event.target.value);
							}}
						/>

						<Divider
							style={{
								width: "100%",
							}}
						/>

						<Typography variant="h2" component="h2">
							Shared
						</Typography>

						<FormControlLabel
							control={
								<Switch
									onChange={(event) => {
										setNoProxy(event.target.checked);
									}}
									checked={noProxy}
									name="noProxySwitch"
									id="noProxySwitch"
								/>
							}
							label="Disable Proxy"
						/>

						<IconButton
							align="center"
							style={{
								width: "100px",
								height: "100px",
								margin: "10px",
							}}
							size="small"
							type="submit"
						>
							<SaveIcon
								style={{
									fontSize: "50px",
								}}
							/>
						</IconButton>
					</Grid>
				</form>
			</Grid>
		</div>
	);
}
