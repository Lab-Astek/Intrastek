import Box from '@mui/material/Box';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';

import { FC } from 'react';


type SelectWrapperProp = {
    value: any,
    children: any[],
    label: string,
    onUpdate?: (arg0: SelectChangeEvent<HTMLInputElement>) => void | undefined
};

const SelectWrapper: FC<SelectWrapperProp> = ({ value, label, onUpdate, children }: SelectWrapperProp) => {
    return (
        <Box sx={{ minWidth: 120 }}>
            <FormControl fullWidth>
                <InputLabel id="select-wrapper-input-label-id">{label}</InputLabel>
                <Select
                    labelId="select-wrapper-input-label-id"
                    id="select-wrapper-input"
                    value={value}
                    label={label}
                    onChange={onUpdate}
                >
                    {children.map((child, index) => <MenuItem key={index} value={index}>{child}</MenuItem>)}
                </Select>
            </FormControl>
        </Box>
    );
}

export default SelectWrapper;