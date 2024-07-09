import { LightMode } from "@mui/icons-material";
import React from "react";
import { MUIWrapperContext } from "./muiWrapper";
import IconButton from '@mui/material/IconButton';

export default function ThemeSwitch() {
    const muiUtils = React.useContext(MUIWrapperContext);
    return (
        <IconButton onClick={muiUtils.toggleColorMode} style={{
            maxHeight: 40,
            minHeight: 40,
        }}><LightMode /></IconButton>
    )
}
