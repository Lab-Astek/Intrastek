import Button from '@mui/material/Button';
import { FC, ReactElement } from 'react';

type ButtonWrapperProps = {
    children: any
    onClick?: () => void
};

const ButtonWrapper: FC<ButtonWrapperProps> = ({ children, onClick }: ButtonWrapperProps): ReactElement => {
    return (
        <Button onClick={onClick} variant="outlined">{children}</Button>
    );
}

export default ButtonWrapper;
