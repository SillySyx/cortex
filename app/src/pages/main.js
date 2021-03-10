import React from 'react';

import { Button } from '../components/button';
import { PasswordList } from '../components/password-list';

export class MainPage extends React.Component {
   constructor(props) {
      super(props);

      this.state = {
         modules: loadModules(),
         view: "passwords",
      };
   }

   render() {
      return (
         <div className="main-layout main-background animation-fade">
            <aside className="main-menu">
               <img className="main-menu-logo" src="icons/brain.svg" alt="" />

               {this.state.modules.map((module, index) => (
                  <Button key={index}
                     active={this.state.view === module.id}
                     clicked={() => this.setState({view: module.id})}>

                     <img src={module.icon}  alt={module.name}  />
                  </Button>
               ))}
            </aside>

            <section className="main-content">
               {this.state.view === "passwords" && 
                  <PasswordList />
               }
            </section>
         </div>
      )
   }
}

function loadModules() {
   return [
      {
         id: "passwords",
         name: "Passwords",
         icon: "icons/password.svg",
      },
      {
         id: "knowledge",
         name: "Knowledge",
         icon: "icons/knowledge.svg",
      },
   ]
}
