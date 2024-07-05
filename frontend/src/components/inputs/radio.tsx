import { ChangeEvent, FC, ReactElement } from 'react';

import Radio from '@mui/material/Radio';
import RadioGroup from '@mui/material/RadioGroup';
import FormControlLabel from '@mui/material/FormControlLabel';
import FormLabel from '@mui/material/FormLabel';

type RadioWrapperProp = {
    defaultValue?: string,
    children: any[],
    name: string,
    row?: boolean | undefined
    onUpdate?: (arg0: ChangeEvent<HTMLInputElement>) => void
};

const RadioWrapper: FC<RadioWrapperProp> = ({ defaultValue = "", children, name, row = false, onUpdate}): ReactElement => {
    let labelId = `radio-buttons-group-activity-label-${name}`
    return <div>
        <FormLabel id={labelId}>{name}</FormLabel>
        <RadioGroup
            aria-labelledby={labelId}
            defaultValue={defaultValue}
            row={row}
            name={`radio-buttons-group-${name}`}
            onChange={onUpdate}
        >
            {children.map((ct, idx) => <FormControlLabel value={ct} control={<Radio />} label={ct} />
            )}
        </RadioGroup>
    </div>
}

export default RadioWrapper;