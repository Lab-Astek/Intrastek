import * as React from 'react';
import Box from '@mui/material/Box';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';

export default function SelectInput({ children }: { children: any[] }) {
    const [age, setAge] = React.useState('');

    const handleChange = (event: SelectChangeEvent) => {
        setAge(event.target.value as string);
    };

    return (
        <Box sx={{ minWidth: 120 }}>
            <FormControl fullWidth>
                <InputLabel id="demo-simple-select-label" className="text-white">Age</InputLabel>
                <Select
                    labelId="demo-simple-select-label"
                    id="select-input"
                    value={age}
                    label="Age"
                    onChange={handleChange}
                    className="text-white bg-blue"
                >
                    {children.map((child, index) => <MenuItem key={index} value={index}>{child}</MenuItem>)}
                </Select>
            </FormControl>
        </Box>
    );
}
