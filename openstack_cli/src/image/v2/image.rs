use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod create;
mod deactivate;
mod delete;
mod file {
    pub(super) mod download;
    pub(super) mod upload;
}
mod list;
mod patch;
mod reactivate;
mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ImageArgs {
    #[command(subcommand)]
    command: ImageCommands,
}

#[derive(Subcommand, Clone)]
pub enum ImageCommands {
    /// Lists public virtual machine (VM) images.
    ///
    /// *Pagination*
    ///
    /// Returns a subset of the larger collection of images and a link that you
    /// can use to get the next set of images. You should always check for the
    /// presence of a next link and use it as the URI in a subsequent HTTP GET
    /// request. You should follow this pattern until a next link is no longer
    /// provided.
    ///
    /// The next link preserves any query parameters that you send in your
    /// initial request. You can use the first link to jump back to the first
    /// page of the collection. If you prefer to paginate through images
    /// manually, use the limit and marker parameters.
    ///
    /// *Query Filters*
    ///
    /// The list operation accepts query parameters to filter the response.
    ///
    /// A client can provide direct comparison filters by using most image
    /// attributes, such as name=Ubuntu, visibility=public, and so on.
    ///
    /// To filter using image tags, use the filter tag (note the singular). To
    /// filter on multiple tags, include each tag separately in the query. For
    /// example, to find images with the tag ready, include tag=ready in your
    /// query string. To find images tagged with ready and approved, include
    /// tag=ready&tag=approved in your query string. (Note that only images
    /// containing both tags will be included in the response.)
    ///
    /// A client cannot use any link in the json-schema, such as self, file, or
    /// schema, to filter the response.
    ///
    /// You can list VM images that have a status of active, queued, or saving.
    ///
    /// *The `in` Operator*
    ///
    /// As a convenience, you may specify several values for any of the
    /// following fields by using the in operator: [container_format,
    /// disk_format, id, name, status]
    ///
    /// For most of these, usage is straight forward. For example, to list
    /// images in queued or saving status, use: `--status "in:saving,queued"`
    ///
    /// To find images in a particular list of image IDs, use: `--id
    /// "in:3afb79c1-131a-4c38-a87c-bc4b801d14e6,2e011209-660f-44b5-baf2-2eb4babae53d"
    ///
    /// Using the in operator with the name property of images can be a bit
    /// trickier, depending upon how creatively you have named your images. The
    /// general rule is that if an image name contains a comma (,), you must
    /// enclose the entire name in quotation marks ("). As usual, you must URL
    /// encode any characters that require it.
    ///
    /// For example, to find images named glass, darkly or share me, you would
    /// use the following filter specification: `--name:
    /// 'in:"glass,%20darkly",share%20me'`
    ///
    /// As with regular filtering by name, you must specify the complete name
    /// you are looking for. Thus, for example, the query `--name
    /// "in:glass,share"` will only match images with the exact name glass or
    /// the exact name share. It will not find an image named glass, darkly or
    /// an image named share me.
    ///
    /// *Size Comparison Filters*
    ///
    /// You can use the size_min and size_max query parameters to filter images
    /// that are greater than or less than the image size. The size, in bytes,
    /// is the size of an image on disk.
    ///
    /// For example, to filter the container to include only images that are
    /// from 1 to 4 MB, set the size_min query parameter to 1048576 and the
    /// size_max query parameter to 4194304.
    ///
    /// *Time Comparison Filters*
    ///
    /// You can use a comparison operator along with the created_at or
    /// updated_at fields to filter your results. Specify the operator first, a
    /// colon (:) as a separator, and then the time in ISO 8601 Format.
    /// Available comparison operators are: [gt, gte, eq, neq, lt, lte]
    #[command(about = "List Images")]
    List(list::ImagesArgs),
    /// Shows details for an image.
    #[command(about = "Show single image")]
    Show(show::ImageArgs),
    /// Creates a catalog record for an operating system disk image. (Since
    /// Image API v2.0)
    ///
    /// The Location response header contains the URI for the image.
    ///
    /// A multiple store backend support is introduced in the Rocky release as
    /// a part of the EXPERIMENTAL Image API v2.8. Since Image API v2.8 a new
    /// header OpenStack-image-store-ids which contains the list of available
    /// stores will be included in response. This header is only included if
    /// multiple backend stores are supported.
    ///
    /// The response body contains the new image entity.
    ///
    /// *Synchronous Postconditions*
    ///
    /// With correct permissions, you can see the image status as queued
    /// through API calls.
    #[command(about = "Create image")]
    Create(create::ImageArgs),
    /// Updates an image.
    #[command(about = "Update image")]
    Set(patch::ImageArgs),
    /// Downloads binary image data. (Since Image API v2.0)
    ///
    /// The response body contains the raw binary data that represents the
    /// actual virtual disk. The Content-Type header contains the
    /// application/octet-stream value. The Content-MD5 header contains an MD5
    /// checksum of the image data. Use this checksum to verify the integrity
    /// of the image data.
    ///
    /// *Preconditions*
    ///
    ///  - The image must exist.
    ///
    /// *Synchronous Postconditions*
    ///
    ///  - You can download the binary image data in your machine if the image
    ///  has image data.
    ///
    ///  - If image data exists, the call returns the HTTP 200 response code
    ///  for a full image download request.
    ///
    ///  - If image data exists, the call returns the HTTP 206 response code
    ///  for a partial download request.
    ///
    ///  - If no image data exists, the call returns the HTTP 204 (No Content)
    ///  response code.
    ///
    ///  - If no image record exists, the call returns the HTTP 404 response
    ///  code for an attempted full image download request.
    ///
    ///  - For an unsatisfiable partial download request, the call returns the
    ///  HTTP 416 response code.
    #[command(about = "Download image data")]
    Download(file::download::FileArgs),
    /// Uploads binary image data.
    ///
    /// These operation may be restricted to administrators. Consult your cloud
    /// operator’s documentation for details.
    ///
    /// *Preconditions* Before you can store binary image data, you must meet
    /// the following preconditions:
    ///
    ///  - The image must exist.
    ///
    ///  - You must set the disk and container formats in the image.
    ///
    ///  - The image status must be queued.
    ///
    ///  - Your image storage quota must be sufficient.
    ///
    ///  - The size of the data that you want to store must not exceed the size
    ///  that the OpenStack Image service allows.
    ///
    /// *Synchronous Postconditions*
    ///
    ///  - With correct permissions, you can see the image status as active
    ///  through API calls.
    ///
    ///  - With correct access, you can see the stored data in the storage
    ///  system that the OpenStack Image Service manages.
    ///
    /// *Troubleshooting*
    ///
    ///  - If you cannot store the data, either your request lacks required
    ///  information or you exceeded your allotted quota. Ensure that you meet
    ///  the preconditions and run the request again. If the request fails
    ///  again, review your API request.
    ///
    ///  - The storage back ends for storing the data must have enough free
    ///  storage space to accommodate the size of the data.
    #[command(about = "Upload image data")]
    Upload(file::upload::FileArgs),
    /// Deletes an image.
    ///
    /// You cannot delete images with the protected attribute set to true
    /// (boolean).
    ///
    /// *Preconditions*
    ///
    ///  - You can delete an image in any status except deleted.
    ///
    ///  - The protected attribute of the image cannot be true.
    ///
    ///  - You have permission to perform image deletion under the configured
    ///  image deletion policy.
    ///
    /// *Synchronous Postconditions*
    ///
    ///  - The response is empty and returns the HTTP 204 response code.
    ///
    ///  - The API deletes the image from the images index.
    ///
    ///  -  If the image has associated binary image data in the storage
    ///  backend, the OpenStack Image service deletes the data.
    #[command(about = "Download image data")]
    Delete(delete::ImageArgs),
    /// Deactivates an image. (Since Image API v2.3)
    ///
    /// By default, this operation is restricted to administrators only.
    #[command(about = "Deactivate image")]
    Deactivate(deactivate::ImageArgs),
    /// Reactivates an image. (Since Image API v2.3)
    ///
    /// By default, this operation is restricted to administrators only
    #[command(about = "Reactivate image")]
    Reactivate(reactivate::ImageArgs),
}

pub struct ImageCommand {
    pub args: ImageArgs,
}

impl ResourceCommands for ImageCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            ImageCommands::List(args) => Box::new(list::ImagesCmd { args: args.clone() }),
            ImageCommands::Show(args) => Box::new(show::ImageCmd { args: args.clone() }),
            ImageCommands::Set(args) => Box::new(patch::ImageCmd { args: args.clone() }),
            ImageCommands::Download(args) => {
                Box::new(file::download::FileCmd { args: args.clone() })
            }
            ImageCommands::Upload(args) => Box::new(file::upload::FileCmd { args: args.clone() }),
            ImageCommands::Create(args) => Box::new(create::ImageCmd { args: args.clone() }),
            ImageCommands::Delete(args) => Box::new(delete::ImageCmd { args: args.clone() }),
            ImageCommands::Deactivate(args) => {
                Box::new(deactivate::ImageCmd { args: args.clone() })
            }
            ImageCommands::Reactivate(args) => {
                Box::new(reactivate::ImageCmd { args: args.clone() })
            }
        }
    }
}
