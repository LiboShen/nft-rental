use crate::*;
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, NFT_METADATA_SPEC,
};

// TODO(libo): Consider minifying it before launch to mainnet.
const DATA_IMAGE_SVG: &str = "data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22598%22%20height%3D%22598%22%20fill%3D%22none%22%3E%3Cg%20clip-path%3D%22url%28%23a%29%22%3E%3Cpath%20fill%3D%22%23fff%22%20d%3D%22M0%200h598v598H0z%22%2F%3E%3Cpath%20fill%3D%22%23343434%22%20d%3D%22m-199%2073%201%205-1%205%201%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205%201%205-1%205h-10v-10h-6l-1-5%201-5h-4l-1-6%201-5h-5l-1-5%201-5h-5v41h-11V62h13l1%205-1%206h4l1%205-1%205h4v10h5l1%205-1%205h5V62h10l1%205-1%206Zm26%200V62h10l1%205-1%206h-10Zm21%2051v10h-31l-1-5%201-5h10V93h-8V83h18l1%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205h11Zm51-31v10h-21l1%205-1%206%201%205-1%205%201%205-1%205h-10V62h31v11h-21l1%205-1%205%201%205-1%205h21Zm57-26-1%206h-15v61h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5h-16V62h41l1%205ZM7%2088l-1%205H3v10h-4v11h-3l1%205-1%205h-4l1%205-1%205h-2v10h-5v10h-16v-10h11l-1-5%201-5-1-5%201-5h-6v-10h-2l-1-6%201-5h-4V93h-4V83h10l1%205-1%205h4l1%205-1%205h4l1%205-1%206%201%205-1%205h4l-1-5%201-5h3l-1-6%201-5h3V93h4V83H6l1%205Zm51%2036v10H47v-10h-3l-1-5%201-5h-4l-1-6%201-5H27v31H17l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5h36v11h5v10h-3l1%205-1%205h-8l1%205-1%205h3l1%205-1%206h4v10h4ZM45%2093l-1-5%201-5-1-5%201-5H27v20h18Zm64%205v16H82l1%205-1%205h22v10H78v-10h-6l-1-5%201-5h-4l-1-6%201-5h4l-1-5%201-5h6V83h26v10h5v5Zm-10%205-1-5%201-5H82l1%205-1%205h17Zm30-15-1%205h5l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10V83h9l1%205Zm31%2015%201%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17V83h21v10h6l1%205-1%205Zm46%2021%201%205-1%205h-20l-1-5%201-5h-5l-1-5%201-5-1-6%201-5-1-5%201-5h-11V83h11l-1-5%201-5-1-6%201-5h10v21h15l1%205-1%205h-15v31h15Zm108%200v10h-36V62h10l1%205-1%206%201%205-1%205%201%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205h26Zm52-26-1%205%201%205-1%206h-27l1%205-1%205h22l1%205-1%205h-26v-10h-6v-10h-4v-11h4V93h6V83h26l1%205-1%205h5l1%205Zm-11%205-1-5%201-5h-17l1%205-1%205h17Zm61%2021%201%205-1%205h-10v-10h-5v-10h-15v10h15l1%205-1%205h-20l-1-5%201-5h-6v-10h6l-1-6%201-5h20V93h-18V83h23l1%205-1%205h5l1%205-1%205%201%205-1%206%201%205-1%205h5Zm50-5-1%205h-5l1%205-1%205h-28l-1-5%201-5h23l-1-5%201-5h-21v-11h-5V93h5V83h29v10h-23v10h20l1%205-1%206h5l1%205Zm53-21v16h-27l1%205-1%205h22v10h-26v-10h-6l-1-5%201-5h-4l-1-6%201-5h4l-1-5%201-5h6V83h26v10h5v5Zm-10%205-1-5%201-5h-17l1%205-1%205h17Zm112-10%201%205-1%205%201%205-1%206h-3v10h-7l1%205-1%205h-20l-1-5%201-5h-7v-10h-4V83h4V73h7l-1-6%201-5h20l1%205-1%206h7v10h3l1%205-1%205Zm-10%2021V83h-4V73h-12v10h-4v31h4v10h12v-10h4Zm62-26v15h-4v21h-3l1%205-1%205h-7l-1-5%201-5h-3v-10h-8l1%205-1%205h-2v10h-8v-10h-2l-1-5%201-5-1-6%201-5h-4l-1-5%201-5-1-5%201-5h10v31h4l-1-6%201-5h2V83h8l1%205-1%205%201%205-1%205h3v11h3V83h11v5Zm19%200v5h5l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5h9v5Zm32%2015v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17V83h21v10h6v10Zm52-5-1%205%201%205-1%206h-27l1%205-1%205h22l1%205-1%205h-26v-10h-6v-10h-4v-11h4V93h6V83h26l1%205-1%205h5l1%205Zm-11%205-1-5%201-5h-17l1%205-1%205h17Zm61-10%201%205-1%205h-10V93h-9l1%205-1%205h-4l1%205-1%206%201%205-1%205h8l1%205-1%205h-26v-10h8V93h-8V83h13l1%205-1%205h5V83h20v10h3Zm50%2026-1%205h-5l1%205-1%205h-28l-1-5%201-5h23l-1-5%201-5h-21v-11h-5V93h5V83h29v10h-23v10h20l1%205-1%206h5l1%205Zm23-31-1%205h4l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5h10l1%205-1%206%201%205-1%205%201%205Zm30%2015v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17V83h21v10h6v10Zm26-30-1-6%201-5h10v11h-10Zm20%2051%201%205-1%205h-31v-10h11l-1-5%201-5-1-6%201-5-1-5%201-5h-8l-1-5%201-5h18v41h10Zm57-16-1%206h-5l1%205-1%205h-5l1%205-1%205h-20v20h-11V83h11v10h5v10h-5v21h15v-10h5v-11h-5V93h-10l-1-5%201-5h15l1%205-1%205h5l1%205-1%205h5l1%205ZM58%20207l1%205-1%205%201%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205%201%205-1%205H48v-10h-6l-1-5%201-5h-4l-1-6%201-5h-5l-1-5%201-5h-5v41H17v-72h13l1%205-1%206h4l1%205-1%205h4v10h5l1%205-1%205h5v-41h10l1%205-1%206Zm26%200v-11h10l1%205-1%206H84Zm21%2051v10H74l-1-5%201-5h10v-31h-8v-10h18l1%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205h11Zm51-31v10h-21l1%205-1%206%201%205-1%205%201%205-1%205h-10v-72h31v11h-21l1%205-1%205%201%205-1%205h21Zm57-26-1%206h-15v61h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5h-16v-11h41l1%205Zm51%2021-1%205h-3v10h-4v11h-3l1%205-1%205h-4l1%205-1%205h-2v10h-5v10h-16v-10h11l-1-5%201-5-1-5%201-5h-6v-10h-2l-1-6%201-5h-4v-10h-4v-10h10l1%205-1%205h4l1%205-1%205h4l1%205-1%206%201%205-1%205h4l-1-5%201-5h3l-1-6%201-5h3v-10h4v-10h10l1%205Zm51%2036v10h-11v-10h-3l-1-5%201-5h-4l-1-6%201-5h-13v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5h36v11h5v10h-3l1%205-1%205h-8l1%205-1%205h3l1%205-1%206h4v10h4Zm-13-31-1-5%201-5-1-5%201-5h-18v20h18Zm64%205v16h-27l1%205-1%205h22v10h-26v-10h-6l-1-5%201-5h-4l-1-6%201-5h4l-1-5%201-5h6v-10h26v10h5v5Zm-10%205-1-5%201-5h-17l1%205-1%205h17Zm30-15-1%205h5l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10v-51h9l1%205Zm31%2015%201%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6l1%205-1%205Zm46%2021%201%205-1%205h-20l-1-5%201-5h-5l-1-5%201-5-1-6%201-5-1-5%201-5h-11v-10h11l-1-5%201-5-1-6%201-5h10v21h15l1%205-1%205h-15v31h15Zm108%200v10h-36v-72h10l1%205-1%206%201%205-1%205%201%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205h26Zm52-26-1%205%201%205-1%206h-27l1%205-1%205h22l1%205-1%205h-26v-10h-6v-10h-4v-11h4v-10h6v-10h26l1%205-1%205h5l1%205Zm-11%205-1-5%201-5h-17l1%205-1%205h17Zm61%2021%201%205-1%205h-10v-10h-5v-10h-15v10h15l1%205-1%205h-20l-1-5%201-5h-6v-10h6l-1-6%201-5h20v-10h-18v-10h23l1%205-1%205h5l1%205-1%205%201%205-1%206%201%205-1%205h5Zm50-5-1%205h-5l1%205-1%205h-28l-1-5%201-5h23l-1-5%201-5h-21v-11h-5v-10h5v-10h29v10h-23v10h20l1%205-1%206h5l1%205Zm53-21v16h-27l1%205-1%205h22v10h-26v-10h-6l-1-5%201-5h-4l-1-6%201-5h4l-1-5%201-5h6v-10h26v10h5v5Zm-10%205-1-5%201-5h-17l1%205-1%205h17Zm112-10%201%205-1%205%201%205-1%206h-3v10h-7l1%205-1%205h-20l-1-5%201-5h-7v-10h-4v-31h4v-10h7l-1-6%201-5h20l1%205-1%206h7v10h3l1%205-1%205Zm-10%2021v-31h-4v-10h-12v10h-4v31h4v10h12v-10h4Zm62-26v15h-4v21h-3l1%205-1%205h-7l-1-5%201-5h-3v-10h-8l1%205-1%205h-2v10h-8v-10h-2l-1-5%201-5-1-6%201-5h-4l-1-5%201-5-1-5%201-5h10v31h4l-1-6%201-5h2v-20h8l1%205-1%205%201%205-1%205h3v11h3v-31h11v5Zm19%200v5h5l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5h9v5Zm32%2015v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6v10Zm52-5-1%205%201%205-1%206h-27l1%205-1%205h22l1%205-1%205h-26v-10h-6v-10h-4v-11h4v-10h6v-10h26l1%205-1%205h5l1%205Zm-11%205-1-5%201-5h-17l1%205-1%205h17Zm61-10%201%205-1%205h-10v-10h-9l1%205-1%205h-4l1%205-1%206%201%205-1%205h8l1%205-1%205h-25l-1-5%201-5h7v-31h-7l-1-5%201-5h12l1%205-1%205h5v-10h20v10h3Zm50%2026-1%205h-5l1%205-1%205h-28l-1-5%201-5h23l-1-5%201-5h-21v-11h-5v-10h5v-10h29v10h-23v10h20l1%205-1%206h5l1%205Zm23-31-1%205h4l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5h10l1%205-1%206%201%205-1%205%201%205Zm30%2015v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6v10Zm26-30-1-6%201-5h10v11h-10Zm20%2051%201%205-1%205h-31v-10h11l-1-5%201-5-1-6%201-5-1-5%201-5h-8l-1-5%201-5h18v41h10Zm57-16-1%206h-5l1%205-1%205h-5l1%205-1%205h-20v20h-10l-1-5%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5h10v10h5v10h-5v21h15v-10h5v-11h-5v-10h-10l-1-5%201-5h15l1%205-1%205h5l1%205-1%205h5l1%205ZM391%20345l1%205-1%205%201%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205%201%205-1%205h-10v-10h-6l-1-5%201-5h-4l-1-6%201-5h-5l-1-5%201-5h-5v41h-11v-72h13l1%205-1%206h4l1%205-1%205h4v10h5l1%205-1%205h5v-41h10l1%205-1%206Zm26%200v-11h10l1%205-1%206h-10Zm21%2051v10h-31l-1-5%201-5h10v-31h-8v-10h18l1%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205h11Zm51-31v10h-21l1%205-1%206%201%205-1%205%201%205-1%205h-10v-72h31v11h-21l1%205-1%205%201%205-1%205h21Zm57-26-1%206h-15v61h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5h-16v-11h41l1%205Zm51%2021-1%205h-3v10h-4v11h-3l1%205-1%205h-4l1%205-1%205h-2v10h-5v10h-16v-10h11l-1-5%201-5-1-5%201-5h-6v-10h-2l-1-6%201-5h-4v-10h-4v-10h10l1%205-1%205h4l1%205-1%205h4l1%205-1%206%201%205-1%205h4l-1-5%201-5h3l-1-6%201-5h3v-10h4v-10h10l1%205Zm51%2036v10h-11v-10h-3l-1-5%201-5h-4l-1-6%201-5h-13v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5h36v11h5v10h-3l1%205-1%205h-8l1%205-1%205h3l1%205-1%206h4v10h4Zm-13-31-1-5%201-5-1-5%201-5h-18v20h18Zm64%205v16h-27l1%205-1%205h22v10h-26v-10h-6l-1-5%201-5h-4l-1-6%201-5h4l-1-5%201-5h6v-10h26v10h5v5Zm-10%205-1-5%201-5h-17l1%205-1%205h17Zm30-15-1%205h5l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10v-51h9l1%205Zm31%2015%201%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6l1%205-1%205Zm46%2021%201%205-1%205h-20l-1-5%201-5h-5l-1-5%201-5-1-6%201-5-1-5%201-5h-11v-10h11l-1-5%201-5-1-6%201-5h10v21h15l1%205-1%205h-15v31h15Zm108%200v10h-36v-72h10l1%205-1%206%201%205-1%205%201%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205h26Zm52-26-1%205%201%205-1%206h-27l1%205-1%205h22l1%205-1%205h-26v-10h-6v-10h-4v-11h4v-10h6v-10h26l1%205-1%205h5l1%205Zm-11%205-1-5%201-5h-17l1%205-1%205h17Zm61%2021%201%205-1%205h-10v-10h-5v-10h-15v10h15l1%205-1%205h-20l-1-5%201-5h-6v-10h6l-1-6%201-5h20v-10h-18v-10h23l1%205-1%205h5l1%205-1%205%201%205-1%206%201%205-1%205h5Zm50-5-1%205h-5l1%205-1%205h-28l-1-5%201-5h23l-1-5%201-5h-21v-11h-5v-10h5v-10h29v10h-23v10h20l1%205-1%206h5l1%205Zm53-21v16h-27l1%205-1%205h22v10h-26v-10h-6l-1-5%201-5h-4l-1-6%201-5h4l-1-5%201-5h6v-10h26v10h5v5Zm-10%205-1-5%201-5h-17l1%205-1%205h17Zm112-10%201%205-1%205%201%205-1%206h-3v10h-7l1%205-1%205h-20l-1-5%201-5h-7v-10h-3l-1-6%201-5-1-5%201-5-1-5%201-5h3v-10h7l-1-6%201-5h20l1%205-1%206h7v10h3l1%205-1%205Zm-10%2021v-31h-4v-10h-12v10h-4v31h4v10h12v-10h4Zm62-26v15h-4v21h-3l1%205-1%205h-7l-1-5%201-5h-3v-10h-8l1%205-1%205h-2v10h-8v-10h-2l-1-5%201-5-1-6%201-5h-4l-1-5%201-5-1-5%201-5h10l1%205-1%205%201%205-1%205%201%205-1%206h4l-1-6%201-5h2v-20h8l1%205-1%205%201%205-1%205h3v11h3v-31h11v5Zm19%200v5h5l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5h9v5Zm32%2015v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6v10Zm52-5-1%205%201%205-1%206h-27l1%205-1%205h22l1%205-1%205h-26v-10h-6v-10h-4v-11h4v-10h6v-10h26l1%205-1%205h5l1%205Zm-11%205-1-5%201-5h-17l1%205-1%205h17Zm61-10%201%205-1%205h-10v-10h-9l1%205-1%205h-4l1%205-1%206%201%205-1%205h8l1%205-1%205h-25l-1-5%201-5h7v-31h-7l-1-5%201-5h12l1%205-1%205h5v-10h20v10h3Zm50%2026-1%205h-5l1%205-1%205h-28l-1-5%201-5h23l-1-5%201-5h-21v-11h-5v-10h5v-10h29v10h-23v10h20l1%205-1%206h5l1%205Zm23-31-1%205h4l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5h10l1%205-1%206%201%205-1%205%201%205Zm30%2015v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6v10Zm26-30-1-6%201-5h10v11h-10Zm20%2051%201%205-1%205h-31v-10h11l-1-5%201-5-1-6%201-5-1-5%201-5h-8l-1-5%201-5h18v41h10Zm57-16-1%206h-5l1%205-1%205h-5l1%205-1%205h-20v20h-10l-1-5%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5h10v10h5v10h-5v21h15v-10h5v-11h-5v-10h-10l-1-5%201-5h15l1%205-1%205h5l1%205-1%205h5l1%205ZM-542%20479l1%205-1%205%201%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205%201%205-1%205h-10v-10h-6l-1-5%201-5h-4l-1-6%201-5h-5l-1-5%201-5h-5v41h-11v-72h13l1%205-1%206h4l1%205-1%205h4v10h5l1%205-1%205h5v-41h10l1%205-1%206Zm26%200v-11h10l1%205-1%206h-10Zm21%2051v10h-31l-1-5%201-5h10v-31h-8v-10h18l1%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205h11Zm51-31v10h-21l1%205-1%206%201%205-1%205%201%205-1%205h-10v-72h31v11h-21l1%205-1%205%201%205-1%205h21Zm57-26-1%206h-15v61h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5h-16v-11h41l1%205Zm51%2021-1%205h-3v10h-4v11h-3l1%205-1%205h-4l1%205-1%205h-2v10h-5v10h-16v-10h11l-1-5%201-5-1-5%201-5h-6v-10h-2l-1-6%201-5h-4v-10h-4v-10h10l1%205-1%205h4l1%205-1%205h4l1%205-1%206%201%205-1%205h4l-1-5%201-5h3l-1-6%201-5h3v-10h4v-10h10l1%205Zm51%2036v10h-11v-10h-3l-1-5%201-5h-4l-1-6%201-5h-13v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5h36v11h5v10h-3l1%205-1%205h-8l1%205-1%205h3l1%205-1%206h4v10h4Zm-13-31-1-5%201-5-1-5%201-5h-18v20h18Zm64%205v16h-27l1%205-1%205h22v10h-26v-10h-6l-1-5%201-5h-4l-1-6%201-5h4l-1-5%201-5h6v-10h26v10h5v5Zm-10%205-1-5%201-5h-17l1%205-1%205h17Zm30-15-1%205h5l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10v-51h9l1%205Zm31%2015%201%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6l1%205-1%205Zm46%2021%201%205-1%205h-20l-1-5%201-5h-5l-1-5%201-5-1-6%201-5-1-5%201-5h-11v-10h11l-1-5%201-5-1-6%201-5h10v21h15l1%205-1%205h-15v31h15Zm108%200v10h-36v-72h10l1%205-1%206%201%205-1%205%201%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205h26Zm52-26-1%205%201%205-1%206H-5l1%205-1%205h22l1%205-1%205H-9v-10h-6v-10h-4v-11h4v-10h6v-10h26l1%205-1%205h5l1%205Zm-11%205-1-5%201-5H-5l1%205-1%205h17Zm61%2021%201%205-1%205H63v-10h-5v-10H43v10h15l1%205-1%205H38l-1-5%201-5h-6v-10h6l-1-6%201-5h20v-10H40v-10h23l1%205-1%205h5l1%205-1%205%201%205-1%206%201%205-1%205h5Zm50-5-1%205h-5l1%205-1%205H89l-1-5%201-5h23l-1-5%201-5H91v-11h-5v-10h5v-10h29v10H97v10h20l1%205-1%206h5l1%205Zm53-21v16h-27l1%205-1%205h22v10h-26v-10h-6l-1-5%201-5h-4l-1-6%201-5h4l-1-5%201-5h6v-10h26v10h5v5Zm-10%205-1-5%201-5h-17l1%205-1%205h17Zm112-10%201%205-1%205%201%205-1%206h-3v10h-7l1%205-1%205h-20l-1-5%201-5h-7v-10h-4v-31h4v-10h7l-1-6%201-5h20l1%205-1%206h7v10h3l1%205-1%205Zm-10%2021v-31h-4v-10h-12v10h-4v31h4v10h12v-10h4Zm62-26v15h-4v21h-3l1%205-1%205h-7l-1-5%201-5h-3v-10h-8l1%205-1%205h-2v10h-8v-10h-2l-1-5%201-5-1-6%201-5h-4l-1-5%201-5-1-5%201-5h10v31h4l-1-6%201-5h2v-20h8l1%205-1%205%201%205-1%205h3v11h3v-31h11v5Zm19%200v5h5l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5h9v5Zm32%2015v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6v10Zm52-5-1%205%201%205-1%206h-27l1%205-1%205h22l1%205-1%205h-26v-10h-6v-10h-4v-11h4v-10h6v-10h26l1%205-1%205h5l1%205Zm-11%205-1-5%201-5h-17l1%205-1%205h17Zm61-10%201%205-1%205h-10v-10h-9l1%205-1%205h-4l1%205-1%206%201%205-1%205h8l1%205-1%205h-26v-10h8v-31h-8v-10h13l1%205-1%205h5v-10h20v10h3Zm50%2026-1%205h-5l1%205-1%205h-28l-1-5%201-5h23l-1-5%201-5h-21v-11h-5v-10h5v-10h29v10h-23v10h20l1%205-1%206h5l1%205Zm23-31-1%205h4l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5h10l1%205-1%206%201%205-1%205%201%205Zm30%2015v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6v10Zm26-30-1-6%201-5h10v11h-10Zm20%2051%201%205-1%205h-31v-10h11l-1-5%201-5-1-6%201-5-1-5%201-5h-8l-1-5%201-5h18v41h10Zm57-16-1%206h-5l1%205-1%205h-5l1%205-1%205h-20v20h-11v-71h11v10h5v10h-5v21h15v-10h5v-11h-5v-10h-10l-1-5%201-5h15l1%205-1%205h5l1%205-1%205h5l1%205ZM-920%20345l1%205-1%205%201%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205%201%205-1%205h-10v-10h-6l-1-5%201-5h-4l-1-6%201-5h-5l-1-5%201-5h-5v41h-11v-72h13l1%205-1%206h4l1%205-1%205h4v10h5l1%205-1%205h5v-41h10l1%205-1%206Zm26%200v-11h10l1%205-1%206h-10Zm21%2051v10h-31l-1-5%201-5h10v-31h-8v-10h18l1%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205h11Zm51-31v10h-21l1%205-1%206%201%205-1%205%201%205-1%205h-10v-72h31v11h-21l1%205-1%205%201%205-1%205h21Zm57-26-1%206h-15v61h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5h-16v-11h41l1%205Zm51%2021-1%205h-3v10h-4v11h-3l1%205-1%205h-4l1%205-1%205h-2v10h-5v10h-16v-10h11l-1-5%201-5-1-5%201-5h-6v-10h-2l-1-6%201-5h-4v-10h-4v-10h10l1%205-1%205h4l1%205-1%205h4l1%205-1%206%201%205-1%205h4l-1-5%201-5h3l-1-6%201-5h3v-10h4v-10h10l1%205Zm51%2036v10h-11v-10h-3l-1-5%201-5h-4l-1-6%201-5h-13v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5h36v11h5v10h-3l1%205-1%205h-8l1%205-1%205h3l1%205-1%206h4v10h4Zm-13-31-1-5%201-5-1-5%201-5h-18v20h18Zm64%205v16h-27l1%205-1%205h22v10h-26v-10h-6l-1-5%201-5h-4l-1-6%201-5h4l-1-5%201-5h6v-10h26v10h5v5Zm-10%205-1-5%201-5h-17l1%205-1%205h17Zm30-15-1%205h5l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10v-51h9l1%205Zm31%2015%201%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6l1%205-1%205Zm46%2021%201%205-1%205h-20l-1-5%201-5h-5l-1-5%201-5-1-6%201-5-1-5%201-5h-11v-10h11l-1-5%201-5-1-6%201-5h10v21h15l1%205-1%205h-15v31h15Zm108%200v10h-36v-72h10l1%205-1%206%201%205-1%205%201%205-1%205%201%205-1%205%201%205-1%206%201%205-1%205h26Zm52-26-1%205%201%205-1%206h-27l1%205-1%205h22l1%205-1%205h-26v-10h-6v-10h-4v-11h4v-10h6v-10h26l1%205-1%205h5l1%205Zm-11%205-1-5%201-5h-17l1%205-1%205h17Zm61%2021%201%205-1%205h-10v-10h-5v-10h-15v10h15l1%205-1%205h-20l-1-5%201-5h-6v-10h6l-1-6%201-5h20v-10h-18v-10h23l1%205-1%205h5l1%205-1%205%201%205-1%206%201%205-1%205h5Zm50-5-1%205h-5l1%205-1%205h-28l-1-5%201-5h23l-1-5%201-5h-21v-11h-5v-10h5v-10h29v10h-23v10h20l1%205-1%206h5l1%205Zm53-21v16h-27l1%205-1%205h22v10h-26v-10h-6l-1-5%201-5h-4l-1-6%201-5h4l-1-5%201-5h6v-10h26v10h5v5Zm-10%205-1-5%201-5h-17l1%205-1%205h17Zm112-10%201%205-1%205%201%205-1%206h-3v10h-7l1%205-1%205h-20l-1-5%201-5h-7v-10h-4v-31h4v-10h7l-1-6%201-5h20l1%205-1%206h7v10h3l1%205-1%205Zm-10%2021v-31h-4v-10h-12v10h-4v31h4v10h12v-10h4Zm62-26v15h-4v21h-3l1%205-1%205h-7l-1-5%201-5h-3v-10h-8l1%205-1%205h-2v10h-8v-10h-2l-1-5%201-5-1-6%201-5h-4l-1-5%201-5-1-5%201-5h10v31h4l-1-6%201-5h2v-20h8l1%205-1%205%201%205-1%205h3v11h3v-31h11v5Zm19%200v5h5l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5h9v5Zm32%2015v31H-7l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6v10Zm52-5-1%205%201%205-1%206H27l1%205-1%205h22l1%205-1%205H23v-10h-6v-10h-4v-11h4v-10h6v-10h26l1%205-1%205h5l1%205Zm-11%205-1-5%201-5H27l1%205-1%205h17Zm61-10%201%205-1%205H95v-10h-9l1%205-1%205h-4l1%205-1%206%201%205-1%205h8l1%205-1%205H64v-10h8v-31h-8v-10h13l1%205-1%205h5v-10h20v10h3Zm50%2026-1%205h-5l1%205-1%205h-28l-1-5%201-5h23l-1-5%201-5h-21v-11h-5v-10h5v-10h29v10h-23v10h20l1%205-1%206h5l1%205Zm23-31-1%205h4l1%205-1%205h-4l1%205-1%206%201%205-1%205%201%205-1%205h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5-1-5%201-5-1-5%201-5-1-6%201-5h10l1%205-1%206%201%205-1%205%201%205Zm30%2015v31h-10l-1-5%201-5-1-5%201-5-1-6%201-5-1-5%201-5h-17v-10h21v10h6v10Zm26-30-1-6%201-5h10v11h-10Zm20%2051%201%205-1%205h-31v-10h11l-1-5%201-5-1-6%201-5-1-5%201-5h-8l-1-5%201-5h18v41h10Zm57-16-1%206h-5l1%205-1%205h-5l1%205-1%205h-20v20h-11v-71h11v10h5v10h-5v21h15v-10h5v-11h-5v-10h-10l-1-5%201-5h15l1%205-1%205h5l1%205-1%205h5l1%205Z%22%2F%3E%3C%2Fg%3E%3Cdefs%3E%3CclipPath%20id%3D%22a%22%3E%3Cpath%20fill%3D%22%23fff%22%20d%3D%22M0%200h598v598H0z%22%2F%3E%3C%2FclipPath%3E%3C%2Fdefs%3E%3C%2Fsvg%3E";

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    // Contract metatdata will be hardcoded for now
    fn nft_metadata(&self) -> NFTContractMetadata {
        NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "NiFTyRent Lease Ownership Token".to_string(),
            symbol: "LEASE".to_string(),
            icon: Some(DATA_IMAGE_SVG.to_string()),
            base_uri: None,
            reference: None,
            reference_hash: None,
        }
    }
}
