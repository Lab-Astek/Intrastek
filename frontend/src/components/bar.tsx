import ThemeSwitch from './themeSwitch';
import { FC, ReactElement } from 'react';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';

type BarProps = {
    title: string
}

const Bar: FC<BarProps> = ({ title }): ReactElement => {
    return <Box sx={{ flexGrow: 1 }}>
        <AppBar position="static">
            {/* <h1 className="text-white">{title}</h1> */}
            <ThemeSwitch />
        </AppBar>
    </Box>
}

export default Bar;