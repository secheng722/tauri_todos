import React, { useEffect } from "react";
import {FaRegTrashAlt} from "react-icons/fa"
const style = {
    li: `flex justify-between bg-slate-200 dark:bg-gray-600 p-4 my-2 capitalize`,
    liComplete: `flex justify-between bg-slate-400 dark:bg-gray-900 p-4 my-2 capitalize`,
    row: `flex`,
    text: `  ml-2 cursor-pointer `,
    textComplete: `  ml-2 cursor-pointer line-through`,
    button: `cursor-pointer flex items-center`,
    checkbox: `checkbox`
    
}
const Todo = ({todo,toggleComplete,deleteTodo}) =>{
    return (
        <li className={todo.complete=="1"?style.liComplete:style.li}>
            <div className={style.row}>
                <input onChange={()=>toggleComplete(todo)} type="checkbox" className={style.checkbox}
                    checked={todo.complete=="1"?'checked':''}/>
                <p onClick={()=>toggleComplete(todo)} className={todo.complete=="1"?style.textComplete:style.text}>{todo.text}</p>
            </div>
            <button onClick={()=>deleteTodo(todo.id)}>{<FaRegTrashAlt />}</button>
        </li>
    )
}

export default Todo