import React, { useEffect, useState } from "react";
import {AiOutlinePlus} from "react-icons/ai";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Todo from "./Todo";
const style = {
  bg: ` h-screen w-screen p-4 bg-gradient-to-r from-[#2F80ED] to-[#1CB5E0]`,
  container: ` bg-slate-100 max-w-[500px] w-full m-auto rounded-md shadow-xl p-4`,
  heading: `text-3xl font-bold text-center text-gray-800 p-2`,
  form: `flex justify-between`,
  input: `border p-2 w-full text-xl`,
  button: `btn  p-4 ml-2 bg-purple-500 hover:bg-purple-600`,
  count: `text-center p-2 `,
}

function App() {
  const [todos,setTodos] =useState([]);
  const [input,setInput] =useState('');
//创建待办
  const createTodo = async(e) => {
    e.preventDefault(e)
    if(input ===''){
      alert('please enter a valid todo')
      return 
    }
    await invoke("insert_new_task",{text:input})
    setInput('')
  }
//从数据库读取待办
  useEffect(()=>{
    const q  = async ()=>{
      let todosArr=JSON.parse(await invoke('get_all_task'));
      setTodos(todosArr)
    }
    return ()=>q()
  })
//更新待办
  const toggleComplete=async(todo)=>{
    await invoke("update_the_task",{id:todo.id,complete:todo.complete==1?0:1});
  }

//删除待办
  const deleteTodo=async(id)=>{
    await invoke("dalete_the_task",{id:id});
  }
  return (
    <div className={style.bg}>
      <div className={style.container}>
        <h3 className={style.heading} >ToDo App</h3>
        <form onSubmit={createTodo} className={style.form}>
          <input value={input} 
                 onChange={(e)=>setInput(e.target.value)}
                 className={style.input} 
                 placeholder="Add ToDo" type="text" />
          <button className={style.button}>{<AiOutlinePlus size={20}/>}</button>
        </form>
        <ul>
          {todos.map((todo,index)=>(
            <Todo key={index} todo={todo} toggleComplete={toggleComplete} 
              deleteTodo={deleteTodo}/>
          ))}
        </ul>
        <p className={style.count}>{`you have ${todos.length} todos`}</p>
      </div>
    </div>
  );
}

export default App;
