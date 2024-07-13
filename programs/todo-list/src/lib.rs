use anchor_lang::prelude::*;

declare_id!("ER22ZVCFZCMwSLkRi66i2aznHhx6u9Pwj7RyTgwvNGjk");

#[program]
pub mod todo_list {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        todo_list.items = vec![];
        Ok(())
    }

    pub fn add_to_item(ctx: Context<AddTodoItem>, description: String) -> Result<()>{
        let todo_list = &mut ctx.accounts.todo_list;
        todo_list.items.push(TodoItem{
            description,
            completed:false,
        });
        Ok(())
    }


    pub fn mark_as_completed(ctx: Context<MarkAsCompleted>, item_index: u64) ->Result<()>{
        let todo_list = &mut ctx.accounts.todo_list;
        if let Some(item) = todo_list.items.get_mut(item_index as usize){
            item.completed = true;
        }
        Ok(())
    }


        pub fn remove_to_item(ctx: Context<RemoveTodoItem>, item_index: u64) -> Result<()>{
            let todo_list = &mut ctx.accounts.todo_list;
            if item_index < todo_list.items.len() as u64{
                todo_list.items.remove(item_index as usize);
            }
            Ok(())
        }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(init, payer = user, space = 8 +1024)]
    pub todo_list: Account<'info, TodoList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TodoItem{
    pub description: String,
    pub completed:bool,
}

#[account]
pub struct TodoList{
    pub items: Vec<TodoItem>,
}

#[derive(Accounts)]
pub struct AddTodoItem<'info>{
    #[account(mut)]
    pub todo_list:Account<'info, TodoList>,
}

#[derive(Accounts)]
pub struct MarkAsCompleted<'info>{
    #[account(mut)]
    pub todo_list:Account<'info, TodoList>,
}

#[derive(Accounts)]
pub struct RemoveTodoItem<'info>{
    #[account(mut)]
    pub todo_list:Account<'info, TodoList>,
}