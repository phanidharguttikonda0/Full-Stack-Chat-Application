import React, { useContext, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import css from './authentication.module.css';

import axios from 'axios';
import { usernameChangeContext } from '../../App';

function SignUp(props) {

    const [username, changeUsername] = useState("") ;
    const [password, changePassword] = useState("") ;
    const [email, changeEmail] = useState("") ;
    const [mobileNumber, changeMobileNumber] = useState("") ;
    const usernamechange = useContext(usernameChangeContext) ;
    const navigate = useNavigate() ;

    const onsubmit = async () => {
        
        try{
            let response = (await axios.post("http://127.0.0.1:8080/sign-up", {
                email: email,
                mobile: mobileNumber,
                username: username,
                password: password
            })) ;
            console.log(response)
            if (response.data) {
                usernamechange(username) ;
                navigate('/')
            }
            else alert("Invalid Credentials")
        }catch(err){
            alert("The Sign-Up details Already Exists")
        }
    }

    const nav = () => {
        navigate('/sign-in') ;
    }

    return (
        <div className={css.sign}>
            <div className={css.main}>
                <input type='text' placeholder='username' value={username} onChange={(event)=> changeUsername(event.target.value)} />
                <input type='password' placeholder='password' value={password} onChange={(event) => changePassword(event.target.value)} />
                <input type='email' placeholder='email' value={email} onChange={(event)=> changeEmail(event.target.value)} />
                <input type='number' placeholder='mobile' value={mobileNumber} onChange={(event) => changeMobileNumber(event.target.value)} />
                <button className={css.link} onClick={nav}> sign in </button>
                <button className={css.btn} onClick={onsubmit}> sign up </button>
            </div>
        </div>
    );
}

export default SignUp;