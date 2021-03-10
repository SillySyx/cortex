import React from 'react';

export class LoginPage extends React.Component {
   constructor(props) {
      super(props);

      this.state = {
         view: "login",
      };
   }

   keyPressed(e) {
      if (e.code === "Enter") {
         this.setState({view: "loading"});

         setTimeout(() => this.props.history.push("/main"), 4000);
      }
   }

   render() {
      return (
         <div className="login-layout login-background animation-fade">
            <img className="login-logo" src="icons/brain.svg" alt="" />

            { this.state.view === "loading" && 
               <img className="login-box login-loader animation-spin" src="icons/loading.svg" alt="" />
            }

            { this.state.view === "login" && 
               <input 
                  className="login-box login-input" 
                  type="password" 
                  placeholder="Enter your password"
                  onKeyUp={e => this.keyPressed(e)} />
            }
         </div>
      )
   }
}