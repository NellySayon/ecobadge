use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor)]
pub struct Eco {
    #[mutable]
    name: String,
}

#[blueprint]
mod ecobadge {
    struct Ecobadge {
        // The resource manager address for the NFTs (badges)
        eco_badge_resource_manager: ResourceManager,
        // A counter for ID generation
        eco_badge_id_counter: u64,
    }

    impl Ecobadge {
        // creates the component and the resource manager    
        pub fn instantiate_ecobadge() -> Global<Ecobadge> {           
           
            let eco_badge_resource_manager = ResourceBuilder::new_integer_non_fungible::<Eco>(OwnerRole::None)
            .metadata(metadata! {
                init {
                    "name" => "Ecosystem Badge", locked;
                    "description" => "Individual badge to access the project section of RadixCharts", locked;
                }
            })
            .burn_roles(burn_roles!{
                burner => rule!(allow_all);
                burner_updater => rule!(deny_all);
            })
            .mint_roles(mint_roles!{
                minter => rule!(allow_all);
                minter_updater => rule!(deny_all);
            })
            .recall_roles(recall_roles!{
                recaller => rule!(allow_all); // needs to be changed!
                recaller_updater => rule!(deny_all);
            })
            .create_with_no_initial_supply();

            // Instantiate an Ecobadge component
            Self {
                eco_badge_resource_manager,
                eco_badge_id_counter: 1,
            }
            .instantiate()            
            .prepare_to_globalize(OwnerRole::None)
            .enable_component_royalties(component_royalties! {
                init {
                    get_eco_badge => Usd(5.into()), updatable;
                    burn_eco_badge => Free, updatable;
                }
            })
            .globalize()
        }

        // Method: Mint a new non fungible token that can be used to represent a badge of ownership of a project in the ecosystem
        // Receives: project name 
        // Returns: the newly minted badge
         pub fn get_eco_badge(&mut self, project: String) -> Bucket {
            let data = Eco {
                name: project,
            };
            // mint a new non fungible token          
            let nft_bucket = self.eco_badge_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::Integer(self.eco_badge_id_counter.into()),
                data,
            );
            self.eco_badge_id_counter += 1;
            
            // return the newly minted badge
            nft_bucket
        }

        // Method: Burns a ecosystem badge if no longer needed
        // Receives: ID of the badge to burn 
        // Returns: none
         pub fn burn_eco_badge(&mut self, nft_bucket: Bucket) {
            // Burn the badge
            nft_bucket.burn();
        }
    }
}
