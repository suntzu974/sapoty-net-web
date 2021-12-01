use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Owner {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OwnerRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OwnerResponse {
    pub id: i32,
    pub name: String,
}

impl OwnerResponse {
    pub fn of(owner: Owner) -> OwnerResponse {
        OwnerResponse {
            id: owner.id,
            name: owner.name,
        }
    }
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Pet {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub animal_type: String,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PetRequest {
    pub name: String,
    pub animal_type: String,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PetResponse {
    pub id: i32,
    pub name: String,
    pub animal_type: String,
    pub color: Option<String>,
}

impl PetResponse {
    pub fn of(pet: Pet) -> PetResponse {
        PetResponse {
            id: pet.id,
            name: pet.name,
            animal_type: pet.animal_type,
            color: pet.color,
        }
    }
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    id: i32,
    name: String,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
}

impl UserResponse {
    pub fn of(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            name: user.name,
        }
    }
}


#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Post {
    id: i32,
    user_id: i32,
    title: String,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PostRequest {
    pub user_id: i32,
    pub title: String,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PostResponse {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
}

impl PostResponse {
    pub fn of(post: Post) -> PostResponse {
        PostResponse {
            id: post.id,
            user_id: post.user_id,
            title: post.title,
        }
    }
}

