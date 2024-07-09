import ThemeSwitch from "./themeSwitch";
import { FC, ReactElement } from "react";
import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import ToolBar from "@mui/material/Toolbar";

type BarProps = {
  title: string;
};

const Bar: FC<BarProps> = ({ title }): ReactElement => {
  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position="static" sx={{ bgcolor: "#cc0000" }}>
        <ToolBar>
          <Typography variant="h6" component="div">
            {title}
          </Typography>
          <Box sx={{ flexGrow: 1 }} />
          <Box sx={{ display: "flex", md: "none", xs: "flex" }}>
            <ThemeSwitch />
          </Box>
        </ToolBar>
      </AppBar>
    </Box>
  );
};

export default Bar;
