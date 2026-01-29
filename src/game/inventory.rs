//! Inventory System
//! 
//! Item management and inventory for the player.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Item rarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

/// Item types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    KeyItem,
    Material,
    Currency,
}

/// Item definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub rarity: ItemRarity,
    pub max_stack: u32,
    pub value: u32,
    pub icon: String,
    pub effects: HashMap<String, f32>,
}

impl Item {
    pub fn new(id: &str, name: &str, item_type: ItemType) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            item_type,
            rarity: ItemRarity::Common,
            max_stack: 99,
            value: 0,
            icon: String::new(),
            effects: HashMap::new(),
        }
    }
    
    pub fn is_stackable(&self) -> bool {
        self.max_stack > 1
    }
    
    pub fn with_rarity(mut self, rarity: ItemRarity) -> Self {
        self.rarity = rarity;
        self
    }
    
    pub fn with_effect(mut self, effect: &str, value: f32) -> Self {
        self.effects.insert(effect.to_string(), value);
        self
    }
}

/// Inventory slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySlot {
    pub item_id: String,
    pub quantity: u32,
}

impl InventorySlot {
    pub fn new(item_id: &str, quantity: u32) -> Self {
        Self {
            item_id: item_id.to_string(),
            quantity,
        }
    }
}

/// Player inventory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub slots: Vec<Option<InventorySlot>>,
    pub max_slots: usize,
    pub gold: u32,
}

impl Inventory {
    pub fn new(max_slots: usize) -> Self {
        Self {
            slots: vec![None; max_slots],
            max_slots,
            gold: 0,
        }
    }
    
    /// Add an item to the inventory
    pub fn add_item(&mut self, item: &Item, quantity: u32) -> u32 {
        let mut remaining = quantity;
        
        // Try to stack with existing items
        if item.is_stackable() {
            for slot in &mut self.slots {
                if remaining == 0 {
                    break;
                }
                
                if let Some(inv_slot) = slot {
                    if inv_slot.item_id == item.id {
                        let space = item.max_stack - inv_slot.quantity;
                        let to_add = remaining.min(space);
                        inv_slot.quantity += to_add;
                        remaining -= to_add;
                    }
                }
            }
        }
        
        // Add to empty slots
        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }
            
            if slot.is_none() {
                let to_add = remaining.min(item.max_stack);
                *slot = Some(InventorySlot::new(&item.id, to_add));
                remaining -= to_add;
            }
        }
        
        // Return amount that couldn't be added
        remaining
    }
    
    /// Remove an item from the inventory
    pub fn remove_item(&mut self, item_id: &str, quantity: u32) -> u32 {
        let mut remaining = quantity;
        
        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }
            
            if let Some(inv_slot) = slot {
                if inv_slot.item_id == item_id {
                    let to_remove = remaining.min(inv_slot.quantity);
                    inv_slot.quantity -= to_remove;
                    remaining -= to_remove;
                    
                    if inv_slot.quantity == 0 {
                        *slot = None;
                    }
                }
            }
        }
        
        // Return amount that was actually removed
        quantity - remaining
    }
    
    /// Check if the inventory contains an item
    pub fn has_item(&self, item_id: &str, quantity: u32) -> bool {
        self.count_item(item_id) >= quantity
    }
    
    /// Count how many of an item the player has
    pub fn count_item(&self, item_id: &str) -> u32 {
        self.slots.iter()
            .filter_map(|s| s.as_ref())
            .filter(|s| s.item_id == item_id)
            .map(|s| s.quantity)
            .sum()
    }
    
    /// Check if there's space for an item
    pub fn has_space_for(&self, item: &Item, quantity: u32) -> bool {
        let mut remaining = quantity;
        
        // Check existing stacks
        if item.is_stackable() {
            for slot in &self.slots {
                if let Some(inv_slot) = slot {
                    if inv_slot.item_id == item.id {
                        let space = item.max_stack - inv_slot.quantity;
                        remaining = remaining.saturating_sub(space);
                    }
                }
            }
        }
        
        // Check empty slots
        let empty_slots = self.slots.iter().filter(|s| s.is_none()).count();
        let slots_needed = (remaining as f32 / item.max_stack as f32).ceil() as usize;
        
        slots_needed <= empty_slots
    }
    
    /// Get used slot count
    pub fn used_slots(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }
    
    /// Is the inventory full?
    pub fn is_full(&self) -> bool {
        self.used_slots() >= self.max_slots
    }
    
    /// Clear the inventory
    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            *slot = None;
        }
    }
}

/// Equipment slots
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Weapon,
    Head,
    Body,
    Accessory1,
    Accessory2,
}

/// Player equipment
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Equipment {
    pub weapon: Option<String>,
    pub head: Option<String>,
    pub body: Option<String>,
    pub accessory1: Option<String>,
    pub accessory2: Option<String>,
}

impl Equipment {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn equip(&mut self, slot: EquipmentSlot, item_id: &str) -> Option<String> {
        let slot_ref = self.get_slot_mut(slot);
        let previous = slot_ref.take();
        *slot_ref = Some(item_id.to_string());
        previous
    }
    
    pub fn unequip(&mut self, slot: EquipmentSlot) -> Option<String> {
        self.get_slot_mut(slot).take()
    }
    
    pub fn get_equipped(&self, slot: EquipmentSlot) -> Option<&str> {
        self.get_slot(slot).as_deref()
    }
    
    fn get_slot(&self, slot: EquipmentSlot) -> &Option<String> {
        match slot {
            EquipmentSlot::Weapon => &self.weapon,
            EquipmentSlot::Head => &self.head,
            EquipmentSlot::Body => &self.body,
            EquipmentSlot::Accessory1 => &self.accessory1,
            EquipmentSlot::Accessory2 => &self.accessory2,
        }
    }
    
    fn get_slot_mut(&mut self, slot: EquipmentSlot) -> &mut Option<String> {
        match slot {
            EquipmentSlot::Weapon => &mut self.weapon,
            EquipmentSlot::Head => &mut self.head,
            EquipmentSlot::Body => &mut self.body,
            EquipmentSlot::Accessory1 => &mut self.accessory1,
            EquipmentSlot::Accessory2 => &mut self.accessory2,
        }
    }
}
