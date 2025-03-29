#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cfdcncfg: (),
    _reserved1: [u8; 0x04],
    cfdcctr: (),
    _reserved2: [u8; 0x04],
    cfdcsts: (),
    _reserved3: [u8; 0x04],
    cfdcerfl: (),
    _reserved4: [u8; 0x78],
    cfdgcfg: Cfdgcfg,
    cfdgctr: Cfdgctr,
    cfdgsts: Cfdgsts,
    cfdgerfl: Cfdgerfl,
    cfdgtsc: Cfdgtsc,
    cfdgaflectr: Cfdgaflectr,
    cfdgaflcfg0: Cfdgaflcfg0,
    _reserved11: [u8; 0x0c],
    cfdrmnb: Cfdrmnb,
    cfdrmnd0: Cfdrmnd0,
    _reserved13: [u8; 0x0c],
    cfdrfcc: [Cfdrfcc; 8],
    cfdrfsts: [Cfdrfsts; 8],
    cfdrfpctr: [Cfdrfpctr; 8],
    cfdcfcc: [Cfdcfcc; 6],
    _reserved17: [u8; 0x48],
    cfdcfcce: [Cfdcfcce; 6],
    _reserved18: [u8; 0x48],
    cfdcfsts: [Cfdcfsts; 6],
    _reserved19: [u8; 0x48],
    cfdcfpctr: [Cfdcfpctr; 6],
    _reserved20: [u8; 0x48],
    cfdfests: Cfdfests,
    cfdffsts: Cfdffsts,
    cfdfmsts: Cfdfmsts,
    cfdrfists: Cfdrfists,
    cfdcfrists: Cfdcfrists,
    cfdcftists: Cfdcftists,
    cfdcfofrists: Cfdcfofrists,
    cfdcfoftists: Cfdcfoftists,
    cfdcfmowsts: Cfdcfmowsts,
    cfdfffsts: Cfdfffsts,
    _reserved30: [u8; 0x08],
    cfdtmc: [Cfdtmc; 8],
    _reserved31: [u8; 0x18],
    cfdtmc32: Cfdtmc,
    cfdtmc33: Cfdtmc,
    cfdtmc34: Cfdtmc,
    cfdtmc35: Cfdtmc,
    cfdtmc36: Cfdtmc,
    cfdtmc37: Cfdtmc,
    cfdtmc38: Cfdtmc,
    cfdtmc39: Cfdtmc,
    _reserved39: [u8; 0x18],
    cfdtmc64: Cfdtmc,
    cfdtmc65: Cfdtmc,
    cfdtmc66: Cfdtmc,
    cfdtmc67: Cfdtmc,
    cfdtmc68: Cfdtmc,
    cfdtmc69: Cfdtmc,
    cfdtmc70: Cfdtmc,
    cfdtmc71: Cfdtmc,
    _reserved47: [u8; 0x18],
    cfdtmc96: Cfdtmc,
    cfdtmc97: Cfdtmc,
    cfdtmc98: Cfdtmc,
    cfdtmc99: Cfdtmc,
    cfdtmc100: Cfdtmc,
    cfdtmc101: Cfdtmc,
    cfdtmc102: Cfdtmc,
    cfdtmc103: Cfdtmc,
    _reserved55: [u8; 0x0498],
    cfdtmsts: [Cfdtmsts; 8],
    _reserved56: [u8; 0x18],
    cfdtmsts32: Cfdtmsts,
    cfdtmsts33: Cfdtmsts,
    cfdtmsts34: Cfdtmsts,
    cfdtmsts35: Cfdtmsts,
    cfdtmsts36: Cfdtmsts,
    cfdtmsts37: Cfdtmsts,
    cfdtmsts38: Cfdtmsts,
    cfdtmsts39: Cfdtmsts,
    _reserved64: [u8; 0x18],
    cfdtmsts64: Cfdtmsts,
    cfdtmsts65: Cfdtmsts,
    cfdtmsts66: Cfdtmsts,
    cfdtmsts67: Cfdtmsts,
    cfdtmsts68: Cfdtmsts,
    cfdtmsts69: Cfdtmsts,
    cfdtmsts70: Cfdtmsts,
    cfdtmsts71: Cfdtmsts,
    _reserved72: [u8; 0x18],
    cfdtmsts96: Cfdtmsts,
    cfdtmsts97: Cfdtmsts,
    cfdtmsts98: Cfdtmsts,
    cfdtmsts99: Cfdtmsts,
    cfdtmsts100: Cfdtmsts,
    cfdtmsts101: Cfdtmsts,
    cfdtmsts102: Cfdtmsts,
    cfdtmsts103: Cfdtmsts,
    _reserved80: [u8; 0x0498],
    cfdtmtrsts: [Cfdtmtrsts; 4],
    _reserved81: [u8; 0x90],
    cfdtmtarsts: [Cfdtmtarsts; 4],
    _reserved82: [u8; 0x90],
    cfdtmtcsts: [Cfdtmtcsts; 4],
    _reserved83: [u8; 0x90],
    cfdtmtasts: [Cfdtmtasts; 4],
    _reserved84: [u8; 0x90],
    cfdtmiec: [Cfdtmiec; 4],
    _reserved85: [u8; 0xa0],
    cfdtxqcc0: [Cfdtxqcc0; 2],
    _reserved86: [u8; 0x18],
    cfdtxqsts0: [Cfdtxqsts0; 2],
    _reserved87: [u8; 0x18],
    cfdtxqpctr0: [Cfdtxqpctr0; 2],
    _reserved88: [u8; 0x18],
    cfdtxqcc1: [Cfdtxqcc1; 2],
    _reserved89: [u8; 0x18],
    cfdtxqsts1: [Cfdtxqsts1; 2],
    _reserved90: [u8; 0x18],
    cfdtxqpctr1: [Cfdtxqpctr1; 2],
    _reserved91: [u8; 0x18],
    cfdtxqcc2: [Cfdtxqcc2; 2],
    _reserved92: [u8; 0x18],
    cfdtxqsts2: [Cfdtxqsts2; 2],
    _reserved93: [u8; 0x18],
    cfdtxqpctr2: [Cfdtxqpctr2; 2],
    _reserved94: [u8; 0x18],
    cfdtxqcc3: [Cfdtxqcc3; 2],
    _reserved95: [u8; 0x18],
    cfdtxqsts3: [Cfdtxqsts3; 2],
    _reserved96: [u8; 0x18],
    cfdtxqpctr3: [Cfdtxqpctr3; 2],
    _reserved97: [u8; 0x18],
    cfdtxqests: Cfdtxqests,
    cfdtxqfists: Cfdtxqfists,
    cfdtxqmsts: Cfdtxqmsts,
    _reserved100: [u8; 0x04],
    cfdtxqists: Cfdtxqists,
    cfdtxqoftists: Cfdtxqoftists,
    cfdtxqofrists: Cfdtxqofrists,
    cfdtxqfsts: Cfdtxqfsts,
    _reserved104: [u8; 0x60],
    cfdthlcc: [Cfdthlcc; 2],
    _reserved105: [u8; 0x18],
    cfdthlsts: [Cfdthlsts; 2],
    _reserved106: [u8; 0x18],
    cfdthlpctr: [Cfdthlpctr; 2],
    _reserved107: [u8; 0xb8],
    cfdgtintsts0: Cfdgtintsts0,
    _reserved108: [u8; 0x04],
    cfdgtstcfg: Cfdgtstcfg,
    cfdgtstctr: Cfdgtstctr,
    _reserved110: [u8; 0x04],
    cfdgfdcfg: Cfdgfdcfg,
    _reserved111: [u8; 0x04],
    cfdglockk: Cfdglockk,
    _reserved112: [u8; 0x10],
    cfdcdtct: Cfdcdtct,
    cfdcdtsts: Cfdcdtsts,
    _reserved114: [u8; 0x08],
    cfdcdttct: Cfdcdttct,
    cfdcdttsts: Cfdcdttsts,
    _reserved116: [u8; 0x08],
    cfdgrintsts: [Cfdgrintsts; 2],
    _reserved117: [u8; 0x28],
    cfdgrstc: Cfdgrstc,
    _reserved118: [u8; 0x7c],
    cfdcdcfg: (),
    _reserved119: [u8; 0x04],
    cfdcfdcfg: (),
    _reserved120: [u8; 0x04],
    cfdcfdctr: (),
    _reserved121: [u8; 0x04],
    cfdcfdsts: (),
    _reserved122: [u8; 0x04],
    cfdcfdcrc: (),
    _reserved123: [u8; 0x08],
    cfdcblct: (),
    _reserved124: [u8; 0x04],
    cfdcblsts: (),
    _reserved125: [u8; 0x03e4],
    cfdgaflid: (),
    _reserved126: [u8; 0x04],
    cfdgaflm: (),
    _reserved127: [u8; 0x04],
    cfdgaflp0: (),
    _reserved128: [u8; 0x04],
    cfdgaflp1: (),
    _reserved129: [u8; 0x07f4],
    cfdrmid_0: (),
    _reserved130: [u8; 0x04],
    cfdrmptr_0: (),
    _reserved131: [u8; 0x04],
    cfdrmfdsts_0: (),
    _reserved132: [u8; 0x04],
    cfdrmdf0__0: (),
    _reserved133: [u8; 0x04],
    cfdrmdf1__0: (),
    _reserved134: [u8; 0x04],
    cfdrmdf2__0: (),
    _reserved135: [u8; 0x04],
    cfdrmdf3__0: (),
    _reserved136: [u8; 0x04],
    cfdrmdf4__0: (),
    _reserved137: [u8; 0x04],
    cfdrmdf5__0: (),
    _reserved138: [u8; 0x04],
    cfdrmdf6__0: (),
    _reserved139: [u8; 0x04],
    cfdrmdf7__0: (),
    _reserved140: [u8; 0x04],
    cfdrmdf8__0: (),
    _reserved141: [u8; 0x04],
    cfdrmdf9__0: (),
    _reserved142: [u8; 0x04],
    cfdrmdf10__0: (),
    _reserved143: [u8; 0x04],
    cfdrmdf11__0: (),
    _reserved144: [u8; 0x04],
    cfdrmdf12__0: (),
    _reserved145: [u8; 0x04],
    cfdrmdf13__0: (),
    _reserved146: [u8; 0x04],
    cfdrmdf14__0: (),
    _reserved147: [u8; 0x04],
    cfdrmdf15__0: (),
    _reserved148: [u8; 0x07b8],
    cfdrmid_1: (),
    _reserved149: [u8; 0x04],
    cfdrmptr_1: (),
    _reserved150: [u8; 0x04],
    cfdrmfdsts_1: (),
    _reserved151: [u8; 0x04],
    cfdrmdf0__1: (),
    _reserved152: [u8; 0x04],
    cfdrmdf1__1: (),
    _reserved153: [u8; 0x04],
    cfdrmdf2__1: (),
    _reserved154: [u8; 0x04],
    cfdrmdf3__1: (),
    _reserved155: [u8; 0x04],
    cfdrmdf4__1: (),
    _reserved156: [u8; 0x04],
    cfdrmdf5__1: (),
    _reserved157: [u8; 0x04],
    cfdrmdf6__1: (),
    _reserved158: [u8; 0x04],
    cfdrmdf7__1: (),
    _reserved159: [u8; 0x04],
    cfdrmdf8__1: (),
    _reserved160: [u8; 0x04],
    cfdrmdf9__1: (),
    _reserved161: [u8; 0x04],
    cfdrmdf10__1: (),
    _reserved162: [u8; 0x04],
    cfdrmdf11__1: (),
    _reserved163: [u8; 0x04],
    cfdrmdf12__1: (),
    _reserved164: [u8; 0x04],
    cfdrmdf13__1: (),
    _reserved165: [u8; 0x04],
    cfdrmdf14__1: (),
    _reserved166: [u8; 0x04],
    cfdrmdf15__1: (),
    _reserved167: [u8; 0x37b8],
    cfdrfid: (),
    _reserved168: [u8; 0x04],
    cfdrfptr: (),
    _reserved169: [u8; 0x04],
    cfdrffdsts: (),
    _reserved170: [u8; 0x04],
    cfdrfdf0: (),
    _reserved171: [u8; 0x04],
    cfdrfdf1: (),
    _reserved172: [u8; 0x04],
    cfdrfdf2: (),
    _reserved173: [u8; 0x04],
    cfdrfdf3: (),
    _reserved174: [u8; 0x04],
    cfdrfdf4: (),
    _reserved175: [u8; 0x04],
    cfdrfdf5: (),
    _reserved176: [u8; 0x04],
    cfdrfdf6: (),
    _reserved177: [u8; 0x04],
    cfdrfdf7: (),
    _reserved178: [u8; 0x04],
    cfdrfdf8: (),
    _reserved179: [u8; 0x04],
    cfdrfdf9: (),
    _reserved180: [u8; 0x04],
    cfdrfdf10: (),
    _reserved181: [u8; 0x04],
    cfdrfdf11: (),
    _reserved182: [u8; 0x04],
    cfdrfdf12: (),
    _reserved183: [u8; 0x04],
    cfdrfdf13: (),
    _reserved184: [u8; 0x04],
    cfdrfdf14: (),
    _reserved185: [u8; 0x04],
    cfdrfdf15: (),
    _reserved186: [u8; 0x03b8],
    cfdcfid_0: (),
    _reserved187: [u8; 0x04],
    cfdcfptr_0: (),
    _reserved188: [u8; 0x04],
    cfdcffdcsts_0: (),
    _reserved189: [u8; 0x04],
    cfdcfdf0_0: (),
    _reserved190: [u8; 0x04],
    cfdcfdf1_0: (),
    _reserved191: [u8; 0x04],
    cfdcfdf2_0: (),
    _reserved192: [u8; 0x04],
    cfdcfdf3_0: (),
    _reserved193: [u8; 0x04],
    cfdcfdf4_0: (),
    _reserved194: [u8; 0x04],
    cfdcfdf5_0: (),
    _reserved195: [u8; 0x04],
    cfdcfdf6_0: (),
    _reserved196: [u8; 0x04],
    cfdcfdf7_0: (),
    _reserved197: [u8; 0x04],
    cfdcfdf8_0: (),
    _reserved198: [u8; 0x04],
    cfdcfdf9_0: (),
    _reserved199: [u8; 0x04],
    cfdcfdf10_0: (),
    _reserved200: [u8; 0x04],
    cfdcfdf11_0: (),
    _reserved201: [u8; 0x04],
    cfdcfdf12_0: (),
    _reserved202: [u8; 0x04],
    cfdcfdf13_0: (),
    _reserved203: [u8; 0x04],
    cfdcfdf14_0: (),
    _reserved204: [u8; 0x04],
    cfdcfdf15_0: (),
    _reserved205: [u8; 0x0138],
    cfdcfid_1: (),
    _reserved206: [u8; 0x04],
    cfdcfptr_1: (),
    _reserved207: [u8; 0x04],
    cfdcffdcsts_1: (),
    _reserved208: [u8; 0x04],
    cfdcfdf0_1: (),
    _reserved209: [u8; 0x04],
    cfdcfdf1_1: (),
    _reserved210: [u8; 0x04],
    cfdcfdf2_1: (),
    _reserved211: [u8; 0x04],
    cfdcfdf3_1: (),
    _reserved212: [u8; 0x04],
    cfdcfdf4_1: (),
    _reserved213: [u8; 0x04],
    cfdcfdf5_1: (),
    _reserved214: [u8; 0x04],
    cfdcfdf6_1: (),
    _reserved215: [u8; 0x04],
    cfdcfdf7_1: (),
    _reserved216: [u8; 0x04],
    cfdcfdf8_1: (),
    _reserved217: [u8; 0x04],
    cfdcfdf9_1: (),
    _reserved218: [u8; 0x04],
    cfdcfdf10_1: (),
    _reserved219: [u8; 0x04],
    cfdcfdf11_1: (),
    _reserved220: [u8; 0x04],
    cfdcfdf12_1: (),
    _reserved221: [u8; 0x04],
    cfdcfdf13_1: (),
    _reserved222: [u8; 0x04],
    cfdcfdf14_1: (),
    _reserved223: [u8; 0x04],
    cfdcfdf15_1: (),
    _reserved224: [u8; 0x1a38],
    cfdthlacc0: (),
    _reserved225: [u8; 0x04],
    cfdthlacc1: (),
    _reserved226: [u8; 0x03fc],
    cfdrpgacc: [Cfdrpgacc; 64],
    _reserved227: [u8; 0x7b00],
    cfdtmid_0: (),
    _reserved228: [u8; 0x04],
    cfdtmptr_0: (),
    _reserved229: [u8; 0x04],
    cfdtmfdctr_0: (),
    _reserved230: [u8; 0x04],
    cfdtmdf0__0: (),
    _reserved231: [u8; 0x04],
    cfdtmdf1__0: (),
    _reserved232: [u8; 0x04],
    cfdtmdf2__0: (),
    _reserved233: [u8; 0x04],
    cfdtmdf3__0: (),
    _reserved234: [u8; 0x04],
    cfdtmdf4__0: (),
    _reserved235: [u8; 0x04],
    cfdtmdf5__0: (),
    _reserved236: [u8; 0x04],
    cfdtmdf6__0: (),
    _reserved237: [u8; 0x04],
    cfdtmdf7__0: (),
    _reserved238: [u8; 0x04],
    cfdtmdf8__0: (),
    _reserved239: [u8; 0x04],
    cfdtmdf9__0: (),
    _reserved240: [u8; 0x04],
    cfdtmdf10__0: (),
    _reserved241: [u8; 0x04],
    cfdtmdf11__0: (),
    _reserved242: [u8; 0x04],
    cfdtmdf12__0: (),
    _reserved243: [u8; 0x04],
    cfdtmdf13__0: (),
    _reserved244: [u8; 0x04],
    cfdtmdf14__0: (),
    _reserved245: [u8; 0x04],
    cfdtmdf15__0: (),
    _reserved246: [u8; 0x0fb8],
    cfdtmid32_0: Cfdtmid0,
    cfdtmptr32_0: Cfdtmptr0,
    cfdtmfdctr32_0: Cfdtmfdctr0,
    cfdtmdf0_32_0: Cfdtmdf0_0,
    cfdtmdf1_32_0: Cfdtmdf1_0,
    cfdtmdf2_32_0: Cfdtmdf2_0,
    cfdtmdf3_32_0: Cfdtmdf3_0,
    cfdtmdf4_32_0: Cfdtmdf4_0,
    cfdtmdf5_32_0: Cfdtmdf5_0,
    cfdtmdf6_32_0: Cfdtmdf6_0,
    cfdtmdf7_32_0: Cfdtmdf7_0,
    cfdtmdf8_32_0: Cfdtmdf8_0,
    cfdtmdf9_32_0: Cfdtmdf9_0,
    cfdtmdf10_32_0: Cfdtmdf10_0,
    cfdtmdf11_32_0: Cfdtmdf11_0,
    cfdtmdf12_32_0: Cfdtmdf12_0,
    cfdtmdf13_32_0: Cfdtmdf13_0,
    cfdtmdf14_32_0: Cfdtmdf14_0,
    cfdtmdf15_32_0: Cfdtmdf15_0,
    _reserved265: [u8; 0x34],
    cfdtmid33_0: Cfdtmid0,
    cfdtmptr33_0: Cfdtmptr0,
    cfdtmfdctr33_0: Cfdtmfdctr0,
    cfdtmdf0_33_0: Cfdtmdf0_0,
    cfdtmdf1_33_0: Cfdtmdf1_0,
    cfdtmdf2_33_0: Cfdtmdf2_0,
    cfdtmdf3_33_0: Cfdtmdf3_0,
    cfdtmdf4_33_0: Cfdtmdf4_0,
    cfdtmdf5_33_0: Cfdtmdf5_0,
    cfdtmdf6_33_0: Cfdtmdf6_0,
    cfdtmdf7_33_0: Cfdtmdf7_0,
    cfdtmdf8_33_0: Cfdtmdf8_0,
    cfdtmdf9_33_0: Cfdtmdf9_0,
    cfdtmdf10_33_0: Cfdtmdf10_0,
    cfdtmdf11_33_0: Cfdtmdf11_0,
    cfdtmdf12_33_0: Cfdtmdf12_0,
    cfdtmdf13_33_0: Cfdtmdf13_0,
    cfdtmdf14_33_0: Cfdtmdf14_0,
    cfdtmdf15_33_0: Cfdtmdf15_0,
    _reserved284: [u8; 0x34],
    cfdtmid34_0: Cfdtmid0,
    cfdtmptr34_0: Cfdtmptr0,
    cfdtmfdctr34_0: Cfdtmfdctr0,
    cfdtmdf0_34_0: Cfdtmdf0_0,
    cfdtmdf1_34_0: Cfdtmdf1_0,
    cfdtmdf2_34_0: Cfdtmdf2_0,
    cfdtmdf3_34_0: Cfdtmdf3_0,
    cfdtmdf4_34_0: Cfdtmdf4_0,
    cfdtmdf5_34_0: Cfdtmdf5_0,
    cfdtmdf6_34_0: Cfdtmdf6_0,
    cfdtmdf7_34_0: Cfdtmdf7_0,
    cfdtmdf8_34_0: Cfdtmdf8_0,
    cfdtmdf9_34_0: Cfdtmdf9_0,
    cfdtmdf10_34_0: Cfdtmdf10_0,
    cfdtmdf11_34_0: Cfdtmdf11_0,
    cfdtmdf12_34_0: Cfdtmdf12_0,
    cfdtmdf13_34_0: Cfdtmdf13_0,
    cfdtmdf14_34_0: Cfdtmdf14_0,
    cfdtmdf15_34_0: Cfdtmdf15_0,
    _reserved303: [u8; 0x34],
    cfdtmid35_0: Cfdtmid0,
    cfdtmptr35_0: Cfdtmptr0,
    cfdtmfdctr35_0: Cfdtmfdctr0,
    cfdtmdf0_35_0: Cfdtmdf0_0,
    cfdtmdf1_35_0: Cfdtmdf1_0,
    cfdtmdf2_35_0: Cfdtmdf2_0,
    cfdtmdf3_35_0: Cfdtmdf3_0,
    cfdtmdf4_35_0: Cfdtmdf4_0,
    cfdtmdf5_35_0: Cfdtmdf5_0,
    cfdtmdf6_35_0: Cfdtmdf6_0,
    cfdtmdf7_35_0: Cfdtmdf7_0,
    cfdtmdf8_35_0: Cfdtmdf8_0,
    cfdtmdf9_35_0: Cfdtmdf9_0,
    cfdtmdf10_35_0: Cfdtmdf10_0,
    cfdtmdf11_35_0: Cfdtmdf11_0,
    cfdtmdf12_35_0: Cfdtmdf12_0,
    cfdtmdf13_35_0: Cfdtmdf13_0,
    cfdtmdf14_35_0: Cfdtmdf14_0,
    cfdtmdf15_35_0: Cfdtmdf15_0,
    _reserved322: [u8; 0x34],
    cfdtmid36_0: Cfdtmid0,
    cfdtmptr36_0: Cfdtmptr0,
    cfdtmfdctr36_0: Cfdtmfdctr0,
    cfdtmdf0_36_0: Cfdtmdf0_0,
    cfdtmdf1_36_0: Cfdtmdf1_0,
    cfdtmdf2_36_0: Cfdtmdf2_0,
    cfdtmdf3_36_0: Cfdtmdf3_0,
    cfdtmdf4_36_0: Cfdtmdf4_0,
    cfdtmdf5_36_0: Cfdtmdf5_0,
    cfdtmdf6_36_0: Cfdtmdf6_0,
    cfdtmdf7_36_0: Cfdtmdf7_0,
    cfdtmdf8_36_0: Cfdtmdf8_0,
    cfdtmdf9_36_0: Cfdtmdf9_0,
    cfdtmdf10_36_0: Cfdtmdf10_0,
    cfdtmdf11_36_0: Cfdtmdf11_0,
    cfdtmdf12_36_0: Cfdtmdf12_0,
    cfdtmdf13_36_0: Cfdtmdf13_0,
    cfdtmdf14_36_0: Cfdtmdf14_0,
    cfdtmdf15_36_0: Cfdtmdf15_0,
    _reserved341: [u8; 0x34],
    cfdtmid37_0: Cfdtmid0,
    cfdtmptr37_0: Cfdtmptr0,
    cfdtmfdctr37_0: Cfdtmfdctr0,
    cfdtmdf0_37_0: Cfdtmdf0_0,
    cfdtmdf1_37_0: Cfdtmdf1_0,
    cfdtmdf2_37_0: Cfdtmdf2_0,
    cfdtmdf3_37_0: Cfdtmdf3_0,
    cfdtmdf4_37_0: Cfdtmdf4_0,
    cfdtmdf5_37_0: Cfdtmdf5_0,
    cfdtmdf6_37_0: Cfdtmdf6_0,
    cfdtmdf7_37_0: Cfdtmdf7_0,
    cfdtmdf8_37_0: Cfdtmdf8_0,
    cfdtmdf9_37_0: Cfdtmdf9_0,
    cfdtmdf10_37_0: Cfdtmdf10_0,
    cfdtmdf11_37_0: Cfdtmdf11_0,
    cfdtmdf12_37_0: Cfdtmdf12_0,
    cfdtmdf13_37_0: Cfdtmdf13_0,
    cfdtmdf14_37_0: Cfdtmdf14_0,
    cfdtmdf15_37_0: Cfdtmdf15_0,
    _reserved360: [u8; 0x34],
    cfdtmid38_0: Cfdtmid0,
    cfdtmptr38_0: Cfdtmptr0,
    cfdtmfdctr38_0: Cfdtmfdctr0,
    cfdtmdf0_38_0: Cfdtmdf0_0,
    cfdtmdf1_38_0: Cfdtmdf1_0,
    cfdtmdf2_38_0: Cfdtmdf2_0,
    cfdtmdf3_38_0: Cfdtmdf3_0,
    cfdtmdf4_38_0: Cfdtmdf4_0,
    cfdtmdf5_38_0: Cfdtmdf5_0,
    cfdtmdf6_38_0: Cfdtmdf6_0,
    cfdtmdf7_38_0: Cfdtmdf7_0,
    cfdtmdf8_38_0: Cfdtmdf8_0,
    cfdtmdf9_38_0: Cfdtmdf9_0,
    cfdtmdf10_38_0: Cfdtmdf10_0,
    cfdtmdf11_38_0: Cfdtmdf11_0,
    cfdtmdf12_38_0: Cfdtmdf12_0,
    cfdtmdf13_38_0: Cfdtmdf13_0,
    cfdtmdf14_38_0: Cfdtmdf14_0,
    cfdtmdf15_38_0: Cfdtmdf15_0,
    _reserved379: [u8; 0x34],
    cfdtmid39_0: Cfdtmid0,
    cfdtmptr39_0: Cfdtmptr0,
    cfdtmfdctr39_0: Cfdtmfdctr0,
    cfdtmdf0_39_0: Cfdtmdf0_0,
    cfdtmdf1_39_0: Cfdtmdf1_0,
    cfdtmdf2_39_0: Cfdtmdf2_0,
    cfdtmdf3_39_0: Cfdtmdf3_0,
    cfdtmdf4_39_0: Cfdtmdf4_0,
    cfdtmdf5_39_0: Cfdtmdf5_0,
    cfdtmdf6_39_0: Cfdtmdf6_0,
    cfdtmdf7_39_0: Cfdtmdf7_0,
    cfdtmdf8_39_0: Cfdtmdf8_0,
    cfdtmdf9_39_0: Cfdtmdf9_0,
    cfdtmdf10_39_0: Cfdtmdf10_0,
    cfdtmdf11_39_0: Cfdtmdf11_0,
    cfdtmdf12_39_0: Cfdtmdf12_0,
    cfdtmdf13_39_0: Cfdtmdf13_0,
    cfdtmdf14_39_0: Cfdtmdf14_0,
    cfdtmdf15_39_0: Cfdtmdf15_0,
    _reserved398: [u8; 0x0c34],
    cfdtmid_1: (),
    _reserved399: [u8; 0x04],
    cfdtmptr_1: (),
    _reserved400: [u8; 0x04],
    cfdtmfdctr_1: (),
    _reserved401: [u8; 0x04],
    cfdtmdf0__1: (),
    _reserved402: [u8; 0x04],
    cfdtmdf1__1: (),
    _reserved403: [u8; 0x04],
    cfdtmdf2__1: (),
    _reserved404: [u8; 0x04],
    cfdtmdf3__1: (),
    _reserved405: [u8; 0x04],
    cfdtmdf4__1: (),
    _reserved406: [u8; 0x04],
    cfdtmdf5__1: (),
    _reserved407: [u8; 0x04],
    cfdtmdf6__1: (),
    _reserved408: [u8; 0x04],
    cfdtmdf7__1: (),
    _reserved409: [u8; 0x04],
    cfdtmdf8__1: (),
    _reserved410: [u8; 0x04],
    cfdtmdf9__1: (),
    _reserved411: [u8; 0x04],
    cfdtmdf10__1: (),
    _reserved412: [u8; 0x04],
    cfdtmdf11__1: (),
    _reserved413: [u8; 0x04],
    cfdtmdf12__1: (),
    _reserved414: [u8; 0x04],
    cfdtmdf13__1: (),
    _reserved415: [u8; 0x04],
    cfdtmdf14__1: (),
    _reserved416: [u8; 0x04],
    cfdtmdf15__1: (),
    _reserved417: [u8; 0x0fb8],
    cfdtmid32_1: Cfdtmid1,
    cfdtmptr32_1: Cfdtmptr1,
    cfdtmfdctr32_1: Cfdtmfdctr1,
    cfdtmdf0_32_1: Cfdtmdf0_1,
    cfdtmdf1_32_1: Cfdtmdf1_1,
    cfdtmdf2_32_1: Cfdtmdf2_1,
    cfdtmdf3_32_1: Cfdtmdf3_1,
    cfdtmdf4_32_1: Cfdtmdf4_1,
    cfdtmdf5_32_1: Cfdtmdf5_1,
    cfdtmdf6_32_1: Cfdtmdf6_1,
    cfdtmdf7_32_1: Cfdtmdf7_1,
    cfdtmdf8_32_1: Cfdtmdf8_1,
    cfdtmdf9_32_1: Cfdtmdf9_1,
    cfdtmdf10_32_1: Cfdtmdf10_1,
    cfdtmdf11_32_1: Cfdtmdf11_1,
    cfdtmdf12_32_1: Cfdtmdf12_1,
    cfdtmdf13_32_1: Cfdtmdf13_1,
    cfdtmdf14_32_1: Cfdtmdf14_1,
    cfdtmdf15_32_1: Cfdtmdf15_1,
    _reserved436: [u8; 0x34],
    cfdtmid33_1: Cfdtmid1,
    cfdtmptr33_1: Cfdtmptr1,
    cfdtmfdctr33_1: Cfdtmfdctr1,
    cfdtmdf0_33_1: Cfdtmdf0_1,
    cfdtmdf1_33_1: Cfdtmdf1_1,
    cfdtmdf2_33_1: Cfdtmdf2_1,
    cfdtmdf3_33_1: Cfdtmdf3_1,
    cfdtmdf4_33_1: Cfdtmdf4_1,
    cfdtmdf5_33_1: Cfdtmdf5_1,
    cfdtmdf6_33_1: Cfdtmdf6_1,
    cfdtmdf7_33_1: Cfdtmdf7_1,
    cfdtmdf8_33_1: Cfdtmdf8_1,
    cfdtmdf9_33_1: Cfdtmdf9_1,
    cfdtmdf10_33_1: Cfdtmdf10_1,
    cfdtmdf11_33_1: Cfdtmdf11_1,
    cfdtmdf12_33_1: Cfdtmdf12_1,
    cfdtmdf13_33_1: Cfdtmdf13_1,
    cfdtmdf14_33_1: Cfdtmdf14_1,
    cfdtmdf15_33_1: Cfdtmdf15_1,
    _reserved455: [u8; 0x34],
    cfdtmid34_1: Cfdtmid1,
    cfdtmptr34_1: Cfdtmptr1,
    cfdtmfdctr34_1: Cfdtmfdctr1,
    cfdtmdf0_34_1: Cfdtmdf0_1,
    cfdtmdf1_34_1: Cfdtmdf1_1,
    cfdtmdf2_34_1: Cfdtmdf2_1,
    cfdtmdf3_34_1: Cfdtmdf3_1,
    cfdtmdf4_34_1: Cfdtmdf4_1,
    cfdtmdf5_34_1: Cfdtmdf5_1,
    cfdtmdf6_34_1: Cfdtmdf6_1,
    cfdtmdf7_34_1: Cfdtmdf7_1,
    cfdtmdf8_34_1: Cfdtmdf8_1,
    cfdtmdf9_34_1: Cfdtmdf9_1,
    cfdtmdf10_34_1: Cfdtmdf10_1,
    cfdtmdf11_34_1: Cfdtmdf11_1,
    cfdtmdf12_34_1: Cfdtmdf12_1,
    cfdtmdf13_34_1: Cfdtmdf13_1,
    cfdtmdf14_34_1: Cfdtmdf14_1,
    cfdtmdf15_34_1: Cfdtmdf15_1,
    _reserved474: [u8; 0x34],
    cfdtmid35_1: Cfdtmid1,
    cfdtmptr35_1: Cfdtmptr1,
    cfdtmfdctr35_1: Cfdtmfdctr1,
    cfdtmdf0_35_1: Cfdtmdf0_1,
    cfdtmdf1_35_1: Cfdtmdf1_1,
    cfdtmdf2_35_1: Cfdtmdf2_1,
    cfdtmdf3_35_1: Cfdtmdf3_1,
    cfdtmdf4_35_1: Cfdtmdf4_1,
    cfdtmdf5_35_1: Cfdtmdf5_1,
    cfdtmdf6_35_1: Cfdtmdf6_1,
    cfdtmdf7_35_1: Cfdtmdf7_1,
    cfdtmdf8_35_1: Cfdtmdf8_1,
    cfdtmdf9_35_1: Cfdtmdf9_1,
    cfdtmdf10_35_1: Cfdtmdf10_1,
    cfdtmdf11_35_1: Cfdtmdf11_1,
    cfdtmdf12_35_1: Cfdtmdf12_1,
    cfdtmdf13_35_1: Cfdtmdf13_1,
    cfdtmdf14_35_1: Cfdtmdf14_1,
    cfdtmdf15_35_1: Cfdtmdf15_1,
    _reserved493: [u8; 0x34],
    cfdtmid36_1: Cfdtmid1,
    cfdtmptr36_1: Cfdtmptr1,
    cfdtmfdctr36_1: Cfdtmfdctr1,
    cfdtmdf0_36_1: Cfdtmdf0_1,
    cfdtmdf1_36_1: Cfdtmdf1_1,
    cfdtmdf2_36_1: Cfdtmdf2_1,
    cfdtmdf3_36_1: Cfdtmdf3_1,
    cfdtmdf4_36_1: Cfdtmdf4_1,
    cfdtmdf5_36_1: Cfdtmdf5_1,
    cfdtmdf6_36_1: Cfdtmdf6_1,
    cfdtmdf7_36_1: Cfdtmdf7_1,
    cfdtmdf8_36_1: Cfdtmdf8_1,
    cfdtmdf9_36_1: Cfdtmdf9_1,
    cfdtmdf10_36_1: Cfdtmdf10_1,
    cfdtmdf11_36_1: Cfdtmdf11_1,
    cfdtmdf12_36_1: Cfdtmdf12_1,
    cfdtmdf13_36_1: Cfdtmdf13_1,
    cfdtmdf14_36_1: Cfdtmdf14_1,
    cfdtmdf15_36_1: Cfdtmdf15_1,
    _reserved512: [u8; 0x34],
    cfdtmid37_1: Cfdtmid1,
    cfdtmptr37_1: Cfdtmptr1,
    cfdtmfdctr37_1: Cfdtmfdctr1,
    cfdtmdf0_37_1: Cfdtmdf0_1,
    cfdtmdf1_37_1: Cfdtmdf1_1,
    cfdtmdf2_37_1: Cfdtmdf2_1,
    cfdtmdf3_37_1: Cfdtmdf3_1,
    cfdtmdf4_37_1: Cfdtmdf4_1,
    cfdtmdf5_37_1: Cfdtmdf5_1,
    cfdtmdf6_37_1: Cfdtmdf6_1,
    cfdtmdf7_37_1: Cfdtmdf7_1,
    cfdtmdf8_37_1: Cfdtmdf8_1,
    cfdtmdf9_37_1: Cfdtmdf9_1,
    cfdtmdf10_37_1: Cfdtmdf10_1,
    cfdtmdf11_37_1: Cfdtmdf11_1,
    cfdtmdf12_37_1: Cfdtmdf12_1,
    cfdtmdf13_37_1: Cfdtmdf13_1,
    cfdtmdf14_37_1: Cfdtmdf14_1,
    cfdtmdf15_37_1: Cfdtmdf15_1,
    _reserved531: [u8; 0x34],
    cfdtmid38_1: Cfdtmid1,
    cfdtmptr38_1: Cfdtmptr1,
    cfdtmfdctr38_1: Cfdtmfdctr1,
    cfdtmdf0_38_1: Cfdtmdf0_1,
    cfdtmdf1_38_1: Cfdtmdf1_1,
    cfdtmdf2_38_1: Cfdtmdf2_1,
    cfdtmdf3_38_1: Cfdtmdf3_1,
    cfdtmdf4_38_1: Cfdtmdf4_1,
    cfdtmdf5_38_1: Cfdtmdf5_1,
    cfdtmdf6_38_1: Cfdtmdf6_1,
    cfdtmdf7_38_1: Cfdtmdf7_1,
    cfdtmdf8_38_1: Cfdtmdf8_1,
    cfdtmdf9_38_1: Cfdtmdf9_1,
    cfdtmdf10_38_1: Cfdtmdf10_1,
    cfdtmdf11_38_1: Cfdtmdf11_1,
    cfdtmdf12_38_1: Cfdtmdf12_1,
    cfdtmdf13_38_1: Cfdtmdf13_1,
    cfdtmdf14_38_1: Cfdtmdf14_1,
    cfdtmdf15_38_1: Cfdtmdf15_1,
    _reserved550: [u8; 0x34],
    cfdtmid39_1: Cfdtmid1,
    cfdtmptr39_1: Cfdtmptr1,
    cfdtmfdctr39_1: Cfdtmfdctr1,
    cfdtmdf0_39_1: Cfdtmdf0_1,
    cfdtmdf1_39_1: Cfdtmdf1_1,
    cfdtmdf2_39_1: Cfdtmdf2_1,
    cfdtmdf3_39_1: Cfdtmdf3_1,
    cfdtmdf4_39_1: Cfdtmdf4_1,
    cfdtmdf5_39_1: Cfdtmdf5_1,
    cfdtmdf6_39_1: Cfdtmdf6_1,
    cfdtmdf7_39_1: Cfdtmdf7_1,
    cfdtmdf8_39_1: Cfdtmdf8_1,
    cfdtmdf9_39_1: Cfdtmdf9_1,
    cfdtmdf10_39_1: Cfdtmdf10_1,
    cfdtmdf11_39_1: Cfdtmdf11_1,
    cfdtmdf12_39_1: Cfdtmdf12_1,
    cfdtmdf13_39_1: Cfdtmdf13_1,
    cfdtmdf14_39_1: Cfdtmdf14_1,
    cfdtmdf15_39_1: Cfdtmdf15_1,
}
impl RegisterBlock {
    ///0x00..0x08 - Channel %s Nominal Bitrate Configuration Register
    #[inline(always)]
    pub const fn cfdcncfg(&self, n: usize) -> &Cfdcncfg {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x00..0x08 - Channel %s Nominal Bitrate Configuration Register
    #[inline(always)]
    pub fn cfdcncfg_iter(&self) -> impl Iterator<Item = &Cfdcncfg> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast()
            })
    }
    ///0x00 - Channel 0 Nominal Bitrate Configuration Register
    #[inline(always)]
    pub const fn cfdc0ncfg(&self) -> &Cfdcncfg {
        self.cfdcncfg(0)
    }
    ///0x10 - Channel 1 Nominal Bitrate Configuration Register
    #[inline(always)]
    pub const fn cfdc1ncfg(&self) -> &Cfdcncfg {
        self.cfdcncfg(1)
    }
    ///0x04..0x0c - Channel %s Control Registers
    #[inline(always)]
    pub const fn cfdcctr(&self, n: usize) -> &Cfdcctr {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x04..0x0c - Channel %s Control Registers
    #[inline(always)]
    pub fn cfdcctr_iter(&self) -> impl Iterator<Item = &Cfdcctr> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(4).add(16 * n).cast()
            })
    }
    ///0x04 - Channel 0 Control Registers
    #[inline(always)]
    pub const fn cfdc0ctr(&self) -> &Cfdcctr {
        self.cfdcctr(0)
    }
    ///0x14 - Channel 1 Control Registers
    #[inline(always)]
    pub const fn cfdc1ctr(&self) -> &Cfdcctr {
        self.cfdcctr(1)
    }
    ///0x08..0x10 - Channel %s Status Registers
    #[inline(always)]
    pub const fn cfdcsts(&self, n: usize) -> &Cfdcsts {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x08..0x10 - Channel %s Status Registers
    #[inline(always)]
    pub fn cfdcsts_iter(&self) -> impl Iterator<Item = &Cfdcsts> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8).add(16 * n).cast()
            })
    }
    ///0x08 - Channel 0 Status Registers
    #[inline(always)]
    pub const fn cfdc0sts(&self) -> &Cfdcsts {
        self.cfdcsts(0)
    }
    ///0x18 - Channel 1 Status Registers
    #[inline(always)]
    pub const fn cfdc1sts(&self) -> &Cfdcsts {
        self.cfdcsts(1)
    }
    ///0x0c..0x14 - Channel %s Error Flag Registers
    #[inline(always)]
    pub const fn cfdcerfl(&self, n: usize) -> &Cfdcerfl {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x0c..0x14 - Channel %s Error Flag Registers
    #[inline(always)]
    pub fn cfdcerfl_iter(&self) -> impl Iterator<Item = &Cfdcerfl> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(12).add(16 * n).cast()
            })
    }
    ///0x0c - Channel 0 Error Flag Registers
    #[inline(always)]
    pub const fn cfdc0erfl(&self) -> &Cfdcerfl {
        self.cfdcerfl(0)
    }
    ///0x1c - Channel 1 Error Flag Registers
    #[inline(always)]
    pub const fn cfdc1erfl(&self) -> &Cfdcerfl {
        self.cfdcerfl(1)
    }
    ///0x84 - Global Configuration Register
    #[inline(always)]
    pub const fn cfdgcfg(&self) -> &Cfdgcfg {
        &self.cfdgcfg
    }
    ///0x88 - Global Control Register
    #[inline(always)]
    pub const fn cfdgctr(&self) -> &Cfdgctr {
        &self.cfdgctr
    }
    ///0x8c - Global Status Register
    #[inline(always)]
    pub const fn cfdgsts(&self) -> &Cfdgsts {
        &self.cfdgsts
    }
    ///0x90 - Global Error Flag Register
    #[inline(always)]
    pub const fn cfdgerfl(&self) -> &Cfdgerfl {
        &self.cfdgerfl
    }
    ///0x94 - Global Timestamp Counter Register
    #[inline(always)]
    pub const fn cfdgtsc(&self) -> &Cfdgtsc {
        &self.cfdgtsc
    }
    ///0x98 - Global Acceptance Filter List Entry Control Register
    #[inline(always)]
    pub const fn cfdgaflectr(&self) -> &Cfdgaflectr {
        &self.cfdgaflectr
    }
    ///0x9c - Global Acceptance Filter List Configuration Register 0
    #[inline(always)]
    pub const fn cfdgaflcfg0(&self) -> &Cfdgaflcfg0 {
        &self.cfdgaflcfg0
    }
    ///0xac - RX Message Buffer Number Register
    #[inline(always)]
    pub const fn cfdrmnb(&self) -> &Cfdrmnb {
        &self.cfdrmnb
    }
    ///0xb0 - RX Message Buffer New Data Register 0
    #[inline(always)]
    pub const fn cfdrmnd0(&self) -> &Cfdrmnd0 {
        &self.cfdrmnd0
    }
    ///0xc0..0xe0 - RX FIFO Configuration/Control Registers %s
    #[inline(always)]
    pub const fn cfdrfcc(&self, n: usize) -> &Cfdrfcc {
        &self.cfdrfcc[n]
    }
    ///Iterator for array of:
    ///0xc0..0xe0 - RX FIFO Configuration/Control Registers %s
    #[inline(always)]
    pub fn cfdrfcc_iter(&self) -> impl Iterator<Item = &Cfdrfcc> {
        self.cfdrfcc.iter()
    }
    ///0xe0..0x100 - RX FIFO Status Registers %s
    #[inline(always)]
    pub const fn cfdrfsts(&self, n: usize) -> &Cfdrfsts {
        &self.cfdrfsts[n]
    }
    ///Iterator for array of:
    ///0xe0..0x100 - RX FIFO Status Registers %s
    #[inline(always)]
    pub fn cfdrfsts_iter(&self) -> impl Iterator<Item = &Cfdrfsts> {
        self.cfdrfsts.iter()
    }
    ///0x100..0x120 - RX FIFO Pointer Control Registers %s
    #[inline(always)]
    pub const fn cfdrfpctr(&self, n: usize) -> &Cfdrfpctr {
        &self.cfdrfpctr[n]
    }
    ///Iterator for array of:
    ///0x100..0x120 - RX FIFO Pointer Control Registers %s
    #[inline(always)]
    pub fn cfdrfpctr_iter(&self) -> impl Iterator<Item = &Cfdrfpctr> {
        self.cfdrfpctr.iter()
    }
    ///0x120..0x138 - Common FIFO Configuration/Control Registers %s
    #[inline(always)]
    pub const fn cfdcfcc(&self, n: usize) -> &Cfdcfcc {
        &self.cfdcfcc[n]
    }
    ///Iterator for array of:
    ///0x120..0x138 - Common FIFO Configuration/Control Registers %s
    #[inline(always)]
    pub fn cfdcfcc_iter(&self) -> impl Iterator<Item = &Cfdcfcc> {
        self.cfdcfcc.iter()
    }
    ///0x180..0x198 - Common FIFO Configuration/Control Enhancement Registers %s
    #[inline(always)]
    pub const fn cfdcfcce(&self, n: usize) -> &Cfdcfcce {
        &self.cfdcfcce[n]
    }
    ///Iterator for array of:
    ///0x180..0x198 - Common FIFO Configuration/Control Enhancement Registers %s
    #[inline(always)]
    pub fn cfdcfcce_iter(&self) -> impl Iterator<Item = &Cfdcfcce> {
        self.cfdcfcce.iter()
    }
    ///0x1e0..0x1f8 - Common FIFO Status Registers %s
    #[inline(always)]
    pub const fn cfdcfsts(&self, n: usize) -> &Cfdcfsts {
        &self.cfdcfsts[n]
    }
    ///Iterator for array of:
    ///0x1e0..0x1f8 - Common FIFO Status Registers %s
    #[inline(always)]
    pub fn cfdcfsts_iter(&self) -> impl Iterator<Item = &Cfdcfsts> {
        self.cfdcfsts.iter()
    }
    ///0x240..0x258 - Common FIFO Pointer Control Registers %s
    #[inline(always)]
    pub const fn cfdcfpctr(&self, n: usize) -> &Cfdcfpctr {
        &self.cfdcfpctr[n]
    }
    ///Iterator for array of:
    ///0x240..0x258 - Common FIFO Pointer Control Registers %s
    #[inline(always)]
    pub fn cfdcfpctr_iter(&self) -> impl Iterator<Item = &Cfdcfpctr> {
        self.cfdcfpctr.iter()
    }
    ///0x2a0 - FIFO Empty Status Register
    #[inline(always)]
    pub const fn cfdfests(&self) -> &Cfdfests {
        &self.cfdfests
    }
    ///0x2a4 - FIFO Full Status Register
    #[inline(always)]
    pub const fn cfdffsts(&self) -> &Cfdffsts {
        &self.cfdffsts
    }
    ///0x2a8 - FIFO Message Lost Status Register
    #[inline(always)]
    pub const fn cfdfmsts(&self) -> &Cfdfmsts {
        &self.cfdfmsts
    }
    ///0x2ac - RX FIFO Interrupt Flag Status Register
    #[inline(always)]
    pub const fn cfdrfists(&self) -> &Cfdrfists {
        &self.cfdrfists
    }
    ///0x2b0 - Common FIFO RX Interrupt Flag Status Register
    #[inline(always)]
    pub const fn cfdcfrists(&self) -> &Cfdcfrists {
        &self.cfdcfrists
    }
    ///0x2b4 - Common FIFO TX Interrupt Flag Status Register
    #[inline(always)]
    pub const fn cfdcftists(&self) -> &Cfdcftists {
        &self.cfdcftists
    }
    ///0x2b8 - Common FIFO One Frame RX Interrupt Flag Status Register
    #[inline(always)]
    pub const fn cfdcfofrists(&self) -> &Cfdcfofrists {
        &self.cfdcfofrists
    }
    ///0x2bc - Common FIFO One Frame TX Interrupt Flag Status Register
    #[inline(always)]
    pub const fn cfdcfoftists(&self) -> &Cfdcfoftists {
        &self.cfdcfoftists
    }
    ///0x2c0 - Common FIFO Message Over Write Status Register
    #[inline(always)]
    pub const fn cfdcfmowsts(&self) -> &Cfdcfmowsts {
        &self.cfdcfmowsts
    }
    ///0x2c4 - FIFO FDC Full Status Register
    #[inline(always)]
    pub const fn cfdfffsts(&self) -> &Cfdfffsts {
        &self.cfdfffsts
    }
    ///0x2d0..0x2d8 - TX Message Buffer Control Registers %s
    #[inline(always)]
    pub const fn cfdtmc(&self, n: usize) -> &Cfdtmc {
        &self.cfdtmc[n]
    }
    ///Iterator for array of:
    ///0x2d0..0x2d8 - TX Message Buffer Control Registers %s
    #[inline(always)]
    pub fn cfdtmc_iter(&self) -> impl Iterator<Item = &Cfdtmc> {
        self.cfdtmc.iter()
    }
    ///0x2f0 - TX Message Buffer Control Registers 32
    #[inline(always)]
    pub const fn cfdtmc32(&self) -> &Cfdtmc {
        &self.cfdtmc32
    }
    ///0x2f0 - TX Message Buffer Control Registers 33
    #[inline(always)]
    pub const fn cfdtmc33(&self) -> &Cfdtmc {
        &self.cfdtmc33
    }
    ///0x2f0 - TX Message Buffer Control Registers 34
    #[inline(always)]
    pub const fn cfdtmc34(&self) -> &Cfdtmc {
        &self.cfdtmc34
    }
    ///0x2f0 - TX Message Buffer Control Registers 35
    #[inline(always)]
    pub const fn cfdtmc35(&self) -> &Cfdtmc {
        &self.cfdtmc35
    }
    ///0x2f0 - TX Message Buffer Control Registers 36
    #[inline(always)]
    pub const fn cfdtmc36(&self) -> &Cfdtmc {
        &self.cfdtmc36
    }
    ///0x2f0 - TX Message Buffer Control Registers 37
    #[inline(always)]
    pub const fn cfdtmc37(&self) -> &Cfdtmc {
        &self.cfdtmc37
    }
    ///0x2f0 - TX Message Buffer Control Registers 38
    #[inline(always)]
    pub const fn cfdtmc38(&self) -> &Cfdtmc {
        &self.cfdtmc38
    }
    ///0x2f0 - TX Message Buffer Control Registers 39
    #[inline(always)]
    pub const fn cfdtmc39(&self) -> &Cfdtmc {
        &self.cfdtmc39
    }
    ///0x310 - TX Message Buffer Control Registers 64
    #[inline(always)]
    pub const fn cfdtmc64(&self) -> &Cfdtmc {
        &self.cfdtmc64
    }
    ///0x310 - TX Message Buffer Control Registers 65
    #[inline(always)]
    pub const fn cfdtmc65(&self) -> &Cfdtmc {
        &self.cfdtmc65
    }
    ///0x310 - TX Message Buffer Control Registers 66
    #[inline(always)]
    pub const fn cfdtmc66(&self) -> &Cfdtmc {
        &self.cfdtmc66
    }
    ///0x310 - TX Message Buffer Control Registers 67
    #[inline(always)]
    pub const fn cfdtmc67(&self) -> &Cfdtmc {
        &self.cfdtmc67
    }
    ///0x310 - TX Message Buffer Control Registers 68
    #[inline(always)]
    pub const fn cfdtmc68(&self) -> &Cfdtmc {
        &self.cfdtmc68
    }
    ///0x310 - TX Message Buffer Control Registers 69
    #[inline(always)]
    pub const fn cfdtmc69(&self) -> &Cfdtmc {
        &self.cfdtmc69
    }
    ///0x310 - TX Message Buffer Control Registers 70
    #[inline(always)]
    pub const fn cfdtmc70(&self) -> &Cfdtmc {
        &self.cfdtmc70
    }
    ///0x310 - TX Message Buffer Control Registers 71
    #[inline(always)]
    pub const fn cfdtmc71(&self) -> &Cfdtmc {
        &self.cfdtmc71
    }
    ///0x330 - TX Message Buffer Control Registers 96
    #[inline(always)]
    pub const fn cfdtmc96(&self) -> &Cfdtmc {
        &self.cfdtmc96
    }
    ///0x330 - TX Message Buffer Control Registers 97
    #[inline(always)]
    pub const fn cfdtmc97(&self) -> &Cfdtmc {
        &self.cfdtmc97
    }
    ///0x330 - TX Message Buffer Control Registers 98
    #[inline(always)]
    pub const fn cfdtmc98(&self) -> &Cfdtmc {
        &self.cfdtmc98
    }
    ///0x330 - TX Message Buffer Control Registers 99
    #[inline(always)]
    pub const fn cfdtmc99(&self) -> &Cfdtmc {
        &self.cfdtmc99
    }
    ///0x330 - TX Message Buffer Control Registers 100
    #[inline(always)]
    pub const fn cfdtmc100(&self) -> &Cfdtmc {
        &self.cfdtmc100
    }
    ///0x330 - TX Message Buffer Control Registers 101
    #[inline(always)]
    pub const fn cfdtmc101(&self) -> &Cfdtmc {
        &self.cfdtmc101
    }
    ///0x330 - TX Message Buffer Control Registers 102
    #[inline(always)]
    pub const fn cfdtmc102(&self) -> &Cfdtmc {
        &self.cfdtmc102
    }
    ///0x330 - TX Message Buffer Control Registers 103
    #[inline(always)]
    pub const fn cfdtmc103(&self) -> &Cfdtmc {
        &self.cfdtmc103
    }
    ///0x7d0..0x7d8 - TX Message Buffer Status Registers %s
    #[inline(always)]
    pub const fn cfdtmsts(&self, n: usize) -> &Cfdtmsts {
        &self.cfdtmsts[n]
    }
    ///Iterator for array of:
    ///0x7d0..0x7d8 - TX Message Buffer Status Registers %s
    #[inline(always)]
    pub fn cfdtmsts_iter(&self) -> impl Iterator<Item = &Cfdtmsts> {
        self.cfdtmsts.iter()
    }
    ///0x7f0 - TX Message Buffer Status Registers 32
    #[inline(always)]
    pub const fn cfdtmsts32(&self) -> &Cfdtmsts {
        &self.cfdtmsts32
    }
    ///0x7f0 - TX Message Buffer Status Registers 33
    #[inline(always)]
    pub const fn cfdtmsts33(&self) -> &Cfdtmsts {
        &self.cfdtmsts33
    }
    ///0x7f0 - TX Message Buffer Status Registers 34
    #[inline(always)]
    pub const fn cfdtmsts34(&self) -> &Cfdtmsts {
        &self.cfdtmsts34
    }
    ///0x7f0 - TX Message Buffer Status Registers 35
    #[inline(always)]
    pub const fn cfdtmsts35(&self) -> &Cfdtmsts {
        &self.cfdtmsts35
    }
    ///0x7f0 - TX Message Buffer Status Registers 36
    #[inline(always)]
    pub const fn cfdtmsts36(&self) -> &Cfdtmsts {
        &self.cfdtmsts36
    }
    ///0x7f0 - TX Message Buffer Status Registers 37
    #[inline(always)]
    pub const fn cfdtmsts37(&self) -> &Cfdtmsts {
        &self.cfdtmsts37
    }
    ///0x7f0 - TX Message Buffer Status Registers 38
    #[inline(always)]
    pub const fn cfdtmsts38(&self) -> &Cfdtmsts {
        &self.cfdtmsts38
    }
    ///0x7f0 - TX Message Buffer Status Registers 39
    #[inline(always)]
    pub const fn cfdtmsts39(&self) -> &Cfdtmsts {
        &self.cfdtmsts39
    }
    ///0x810 - TX Message Buffer Status Registers 64
    #[inline(always)]
    pub const fn cfdtmsts64(&self) -> &Cfdtmsts {
        &self.cfdtmsts64
    }
    ///0x810 - TX Message Buffer Status Registers 65
    #[inline(always)]
    pub const fn cfdtmsts65(&self) -> &Cfdtmsts {
        &self.cfdtmsts65
    }
    ///0x810 - TX Message Buffer Status Registers 66
    #[inline(always)]
    pub const fn cfdtmsts66(&self) -> &Cfdtmsts {
        &self.cfdtmsts66
    }
    ///0x810 - TX Message Buffer Status Registers 67
    #[inline(always)]
    pub const fn cfdtmsts67(&self) -> &Cfdtmsts {
        &self.cfdtmsts67
    }
    ///0x810 - TX Message Buffer Status Registers 68
    #[inline(always)]
    pub const fn cfdtmsts68(&self) -> &Cfdtmsts {
        &self.cfdtmsts68
    }
    ///0x810 - TX Message Buffer Status Registers 69
    #[inline(always)]
    pub const fn cfdtmsts69(&self) -> &Cfdtmsts {
        &self.cfdtmsts69
    }
    ///0x810 - TX Message Buffer Status Registers 70
    #[inline(always)]
    pub const fn cfdtmsts70(&self) -> &Cfdtmsts {
        &self.cfdtmsts70
    }
    ///0x810 - TX Message Buffer Status Registers 71
    #[inline(always)]
    pub const fn cfdtmsts71(&self) -> &Cfdtmsts {
        &self.cfdtmsts71
    }
    ///0x830 - TX Message Buffer Status Registers 96
    #[inline(always)]
    pub const fn cfdtmsts96(&self) -> &Cfdtmsts {
        &self.cfdtmsts96
    }
    ///0x830 - TX Message Buffer Status Registers 97
    #[inline(always)]
    pub const fn cfdtmsts97(&self) -> &Cfdtmsts {
        &self.cfdtmsts97
    }
    ///0x830 - TX Message Buffer Status Registers 98
    #[inline(always)]
    pub const fn cfdtmsts98(&self) -> &Cfdtmsts {
        &self.cfdtmsts98
    }
    ///0x830 - TX Message Buffer Status Registers 99
    #[inline(always)]
    pub const fn cfdtmsts99(&self) -> &Cfdtmsts {
        &self.cfdtmsts99
    }
    ///0x830 - TX Message Buffer Status Registers 100
    #[inline(always)]
    pub const fn cfdtmsts100(&self) -> &Cfdtmsts {
        &self.cfdtmsts100
    }
    ///0x830 - TX Message Buffer Status Registers 101
    #[inline(always)]
    pub const fn cfdtmsts101(&self) -> &Cfdtmsts {
        &self.cfdtmsts101
    }
    ///0x830 - TX Message Buffer Status Registers 102
    #[inline(always)]
    pub const fn cfdtmsts102(&self) -> &Cfdtmsts {
        &self.cfdtmsts102
    }
    ///0x830 - TX Message Buffer Status Registers 103
    #[inline(always)]
    pub const fn cfdtmsts103(&self) -> &Cfdtmsts {
        &self.cfdtmsts103
    }
    ///0xcd0..0xce0 - TX Message Buffer Transmission Request Status Register %s
    #[inline(always)]
    pub const fn cfdtmtrsts(&self, n: usize) -> &Cfdtmtrsts {
        &self.cfdtmtrsts[n]
    }
    ///Iterator for array of:
    ///0xcd0..0xce0 - TX Message Buffer Transmission Request Status Register %s
    #[inline(always)]
    pub fn cfdtmtrsts_iter(&self) -> impl Iterator<Item = &Cfdtmtrsts> {
        self.cfdtmtrsts.iter()
    }
    ///0xd70..0xd80 - TX Message Buffer Transmission Abort Request Status Register %s
    #[inline(always)]
    pub const fn cfdtmtarsts(&self, n: usize) -> &Cfdtmtarsts {
        &self.cfdtmtarsts[n]
    }
    ///Iterator for array of:
    ///0xd70..0xd80 - TX Message Buffer Transmission Abort Request Status Register %s
    #[inline(always)]
    pub fn cfdtmtarsts_iter(&self) -> impl Iterator<Item = &Cfdtmtarsts> {
        self.cfdtmtarsts.iter()
    }
    ///0xe10..0xe20 - TX Message Buffer Transmission Completion Status Register %s
    #[inline(always)]
    pub const fn cfdtmtcsts(&self, n: usize) -> &Cfdtmtcsts {
        &self.cfdtmtcsts[n]
    }
    ///Iterator for array of:
    ///0xe10..0xe20 - TX Message Buffer Transmission Completion Status Register %s
    #[inline(always)]
    pub fn cfdtmtcsts_iter(&self) -> impl Iterator<Item = &Cfdtmtcsts> {
        self.cfdtmtcsts.iter()
    }
    ///0xeb0..0xec0 - TX Message Buffer Transmission Abort Status Register %s
    #[inline(always)]
    pub const fn cfdtmtasts(&self, n: usize) -> &Cfdtmtasts {
        &self.cfdtmtasts[n]
    }
    ///Iterator for array of:
    ///0xeb0..0xec0 - TX Message Buffer Transmission Abort Status Register %s
    #[inline(always)]
    pub fn cfdtmtasts_iter(&self) -> impl Iterator<Item = &Cfdtmtasts> {
        self.cfdtmtasts.iter()
    }
    ///0xf50..0xf60 - TX Message Buffer Interrupt Enable Configuration Register %s
    #[inline(always)]
    pub const fn cfdtmiec(&self, n: usize) -> &Cfdtmiec {
        &self.cfdtmiec[n]
    }
    ///Iterator for array of:
    ///0xf50..0xf60 - TX Message Buffer Interrupt Enable Configuration Register %s
    #[inline(always)]
    pub fn cfdtmiec_iter(&self) -> impl Iterator<Item = &Cfdtmiec> {
        self.cfdtmiec.iter()
    }
    ///0x1000..0x1008 - TX Queue Configuration/Control Registers 0%s
    #[inline(always)]
    pub const fn cfdtxqcc0(&self, n: usize) -> &Cfdtxqcc0 {
        &self.cfdtxqcc0[n]
    }
    ///Iterator for array of:
    ///0x1000..0x1008 - TX Queue Configuration/Control Registers 0%s
    #[inline(always)]
    pub fn cfdtxqcc0_iter(&self) -> impl Iterator<Item = &Cfdtxqcc0> {
        self.cfdtxqcc0.iter()
    }
    ///0x1020..0x1028 - TX Queue Status Registers 0%s
    #[inline(always)]
    pub const fn cfdtxqsts0(&self, n: usize) -> &Cfdtxqsts0 {
        &self.cfdtxqsts0[n]
    }
    ///Iterator for array of:
    ///0x1020..0x1028 - TX Queue Status Registers 0%s
    #[inline(always)]
    pub fn cfdtxqsts0_iter(&self) -> impl Iterator<Item = &Cfdtxqsts0> {
        self.cfdtxqsts0.iter()
    }
    ///0x1040..0x1048 - TX Queue Pointer Control Registers 0%s
    #[inline(always)]
    pub const fn cfdtxqpctr0(&self, n: usize) -> &Cfdtxqpctr0 {
        &self.cfdtxqpctr0[n]
    }
    ///Iterator for array of:
    ///0x1040..0x1048 - TX Queue Pointer Control Registers 0%s
    #[inline(always)]
    pub fn cfdtxqpctr0_iter(&self) -> impl Iterator<Item = &Cfdtxqpctr0> {
        self.cfdtxqpctr0.iter()
    }
    ///0x1060..0x1068 - TX Queue Configuration/Control Registers 1%s
    #[inline(always)]
    pub const fn cfdtxqcc1(&self, n: usize) -> &Cfdtxqcc1 {
        &self.cfdtxqcc1[n]
    }
    ///Iterator for array of:
    ///0x1060..0x1068 - TX Queue Configuration/Control Registers 1%s
    #[inline(always)]
    pub fn cfdtxqcc1_iter(&self) -> impl Iterator<Item = &Cfdtxqcc1> {
        self.cfdtxqcc1.iter()
    }
    ///0x1080..0x1088 - TX Queue Status Registers 1%s
    #[inline(always)]
    pub const fn cfdtxqsts1(&self, n: usize) -> &Cfdtxqsts1 {
        &self.cfdtxqsts1[n]
    }
    ///Iterator for array of:
    ///0x1080..0x1088 - TX Queue Status Registers 1%s
    #[inline(always)]
    pub fn cfdtxqsts1_iter(&self) -> impl Iterator<Item = &Cfdtxqsts1> {
        self.cfdtxqsts1.iter()
    }
    ///0x10a0..0x10a8 - TX Queue Pointer Control Registers 1%s
    #[inline(always)]
    pub const fn cfdtxqpctr1(&self, n: usize) -> &Cfdtxqpctr1 {
        &self.cfdtxqpctr1[n]
    }
    ///Iterator for array of:
    ///0x10a0..0x10a8 - TX Queue Pointer Control Registers 1%s
    #[inline(always)]
    pub fn cfdtxqpctr1_iter(&self) -> impl Iterator<Item = &Cfdtxqpctr1> {
        self.cfdtxqpctr1.iter()
    }
    ///0x10c0..0x10c8 - TX Queue Configuration/Control Registers 2%s
    #[inline(always)]
    pub const fn cfdtxqcc2(&self, n: usize) -> &Cfdtxqcc2 {
        &self.cfdtxqcc2[n]
    }
    ///Iterator for array of:
    ///0x10c0..0x10c8 - TX Queue Configuration/Control Registers 2%s
    #[inline(always)]
    pub fn cfdtxqcc2_iter(&self) -> impl Iterator<Item = &Cfdtxqcc2> {
        self.cfdtxqcc2.iter()
    }
    ///0x10e0..0x10e8 - TX Queue Status Registers 2%s
    #[inline(always)]
    pub const fn cfdtxqsts2(&self, n: usize) -> &Cfdtxqsts2 {
        &self.cfdtxqsts2[n]
    }
    ///Iterator for array of:
    ///0x10e0..0x10e8 - TX Queue Status Registers 2%s
    #[inline(always)]
    pub fn cfdtxqsts2_iter(&self) -> impl Iterator<Item = &Cfdtxqsts2> {
        self.cfdtxqsts2.iter()
    }
    ///0x1100..0x1108 - TX Queue Pointer Control Registers 2%s
    #[inline(always)]
    pub const fn cfdtxqpctr2(&self, n: usize) -> &Cfdtxqpctr2 {
        &self.cfdtxqpctr2[n]
    }
    ///Iterator for array of:
    ///0x1100..0x1108 - TX Queue Pointer Control Registers 2%s
    #[inline(always)]
    pub fn cfdtxqpctr2_iter(&self) -> impl Iterator<Item = &Cfdtxqpctr2> {
        self.cfdtxqpctr2.iter()
    }
    ///0x1120..0x1128 - TX Queue Configuration/Control Registers 3%s
    #[inline(always)]
    pub const fn cfdtxqcc3(&self, n: usize) -> &Cfdtxqcc3 {
        &self.cfdtxqcc3[n]
    }
    ///Iterator for array of:
    ///0x1120..0x1128 - TX Queue Configuration/Control Registers 3%s
    #[inline(always)]
    pub fn cfdtxqcc3_iter(&self) -> impl Iterator<Item = &Cfdtxqcc3> {
        self.cfdtxqcc3.iter()
    }
    ///0x1140..0x1148 - TX Queue Status Registers 3%s
    #[inline(always)]
    pub const fn cfdtxqsts3(&self, n: usize) -> &Cfdtxqsts3 {
        &self.cfdtxqsts3[n]
    }
    ///Iterator for array of:
    ///0x1140..0x1148 - TX Queue Status Registers 3%s
    #[inline(always)]
    pub fn cfdtxqsts3_iter(&self) -> impl Iterator<Item = &Cfdtxqsts3> {
        self.cfdtxqsts3.iter()
    }
    ///0x1160..0x1168 - TX Queue Pointer Control Registers 3%s
    #[inline(always)]
    pub const fn cfdtxqpctr3(&self, n: usize) -> &Cfdtxqpctr3 {
        &self.cfdtxqpctr3[n]
    }
    ///Iterator for array of:
    ///0x1160..0x1168 - TX Queue Pointer Control Registers 3%s
    #[inline(always)]
    pub fn cfdtxqpctr3_iter(&self) -> impl Iterator<Item = &Cfdtxqpctr3> {
        self.cfdtxqpctr3.iter()
    }
    ///0x1180 - TX Queue Empty Status Register
    #[inline(always)]
    pub const fn cfdtxqests(&self) -> &Cfdtxqests {
        &self.cfdtxqests
    }
    ///0x1184 - TX Queue Full Interrupt Status Register
    #[inline(always)]
    pub const fn cfdtxqfists(&self) -> &Cfdtxqfists {
        &self.cfdtxqfists
    }
    ///0x1188 - TX Queue Message Lost Status Register
    #[inline(always)]
    pub const fn cfdtxqmsts(&self) -> &Cfdtxqmsts {
        &self.cfdtxqmsts
    }
    ///0x1190 - TX Queue Interrupt Status Register
    #[inline(always)]
    pub const fn cfdtxqists(&self) -> &Cfdtxqists {
        &self.cfdtxqists
    }
    ///0x1194 - TX Queue One Frame TX Interrupt Status Register
    #[inline(always)]
    pub const fn cfdtxqoftists(&self) -> &Cfdtxqoftists {
        &self.cfdtxqoftists
    }
    ///0x1198 - TX Queue One Frame RX Interrupt Status Register
    #[inline(always)]
    pub const fn cfdtxqofrists(&self) -> &Cfdtxqofrists {
        &self.cfdtxqofrists
    }
    ///0x119c - TX Queue Full Status Register
    #[inline(always)]
    pub const fn cfdtxqfsts(&self) -> &Cfdtxqfsts {
        &self.cfdtxqfsts
    }
    ///0x1200..0x1208 - TX History List Configuration/Control Register %s
    #[inline(always)]
    pub const fn cfdthlcc(&self, n: usize) -> &Cfdthlcc {
        &self.cfdthlcc[n]
    }
    ///Iterator for array of:
    ///0x1200..0x1208 - TX History List Configuration/Control Register %s
    #[inline(always)]
    pub fn cfdthlcc_iter(&self) -> impl Iterator<Item = &Cfdthlcc> {
        self.cfdthlcc.iter()
    }
    ///0x1220..0x1228 - TX History List Status Register %s
    #[inline(always)]
    pub const fn cfdthlsts(&self, n: usize) -> &Cfdthlsts {
        &self.cfdthlsts[n]
    }
    ///Iterator for array of:
    ///0x1220..0x1228 - TX History List Status Register %s
    #[inline(always)]
    pub fn cfdthlsts_iter(&self) -> impl Iterator<Item = &Cfdthlsts> {
        self.cfdthlsts.iter()
    }
    ///0x1240..0x1248 - TX History List Pointer Control Registers %s
    #[inline(always)]
    pub const fn cfdthlpctr(&self, n: usize) -> &Cfdthlpctr {
        &self.cfdthlpctr[n]
    }
    ///Iterator for array of:
    ///0x1240..0x1248 - TX History List Pointer Control Registers %s
    #[inline(always)]
    pub fn cfdthlpctr_iter(&self) -> impl Iterator<Item = &Cfdthlpctr> {
        self.cfdthlpctr.iter()
    }
    ///0x1300 - Global TX Interrupt Status Register 0
    #[inline(always)]
    pub const fn cfdgtintsts0(&self) -> &Cfdgtintsts0 {
        &self.cfdgtintsts0
    }
    ///0x1308 - Global Test Configuration Register
    #[inline(always)]
    pub const fn cfdgtstcfg(&self) -> &Cfdgtstcfg {
        &self.cfdgtstcfg
    }
    ///0x130c - Global Test Control Register
    #[inline(always)]
    pub const fn cfdgtstctr(&self) -> &Cfdgtstctr {
        &self.cfdgtstctr
    }
    ///0x1314 - Global FD Configuration Register
    #[inline(always)]
    pub const fn cfdgfdcfg(&self) -> &Cfdgfdcfg {
        &self.cfdgfdcfg
    }
    ///0x131c - Global Lock Key Register
    #[inline(always)]
    pub const fn cfdglockk(&self) -> &Cfdglockk {
        &self.cfdglockk
    }
    ///0x1330 - DMA Transfer Control Register
    #[inline(always)]
    pub const fn cfdcdtct(&self) -> &Cfdcdtct {
        &self.cfdcdtct
    }
    ///0x1334 - DMA Transfer Status Register
    #[inline(always)]
    pub const fn cfdcdtsts(&self) -> &Cfdcdtsts {
        &self.cfdcdtsts
    }
    ///0x1340 - DMA TX Transfer Control Register
    #[inline(always)]
    pub const fn cfdcdttct(&self) -> &Cfdcdttct {
        &self.cfdcdttct
    }
    ///0x1344 - DMA TX Transfer Status Register
    #[inline(always)]
    pub const fn cfdcdttsts(&self) -> &Cfdcdttsts {
        &self.cfdcdttsts
    }
    ///0x1350..0x1358 - Global RX Interrupt Status Register %s
    #[inline(always)]
    pub const fn cfdgrintsts(&self, n: usize) -> &Cfdgrintsts {
        &self.cfdgrintsts[n]
    }
    ///Iterator for array of:
    ///0x1350..0x1358 - Global RX Interrupt Status Register %s
    #[inline(always)]
    pub fn cfdgrintsts_iter(&self) -> impl Iterator<Item = &Cfdgrintsts> {
        self.cfdgrintsts.iter()
    }
    ///0x1380 - Global SW reset Register
    #[inline(always)]
    pub const fn cfdgrstc(&self) -> &Cfdgrstc {
        &self.cfdgrstc
    }
    ///0x1400..0x1408 - Channel %s Data Bitrate Configuration Register
    #[inline(always)]
    pub const fn cfdcdcfg(&self, n: usize) -> &Cfdcdcfg {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(5120).add(32 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1400..0x1408 - Channel %s Data Bitrate Configuration Register
    #[inline(always)]
    pub fn cfdcdcfg_iter(&self) -> impl Iterator<Item = &Cfdcdcfg> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(5120).add(32 * n).cast()
            })
    }
    ///0x1400 - Channel 0 Data Bitrate Configuration Register
    #[inline(always)]
    pub const fn cfdc0dcfg(&self) -> &Cfdcdcfg {
        self.cfdcdcfg(0)
    }
    ///0x1420 - Channel 1 Data Bitrate Configuration Register
    #[inline(always)]
    pub const fn cfdc1dcfg(&self) -> &Cfdcdcfg {
        self.cfdcdcfg(1)
    }
    ///0x1404..0x140c - Channel %s CAN-FD Configuration Register
    #[inline(always)]
    pub const fn cfdcfdcfg(&self, n: usize) -> &Cfdcfdcfg {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(5124).add(32 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1404..0x140c - Channel %s CAN-FD Configuration Register
    #[inline(always)]
    pub fn cfdcfdcfg_iter(&self) -> impl Iterator<Item = &Cfdcfdcfg> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(5124).add(32 * n).cast()
            })
    }
    ///0x1404 - Channel 0 CAN-FD Configuration Register
    #[inline(always)]
    pub const fn cfdc0fdcfg(&self) -> &Cfdcfdcfg {
        self.cfdcfdcfg(0)
    }
    ///0x1424 - Channel 1 CAN-FD Configuration Register
    #[inline(always)]
    pub const fn cfdc1fdcfg(&self) -> &Cfdcfdcfg {
        self.cfdcfdcfg(1)
    }
    ///0x1408..0x1410 - Channel %s CANFD Control Register
    #[inline(always)]
    pub const fn cfdcfdctr(&self, n: usize) -> &Cfdcfdctr {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(5128).add(32 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1408..0x1410 - Channel %s CANFD Control Register
    #[inline(always)]
    pub fn cfdcfdctr_iter(&self) -> impl Iterator<Item = &Cfdcfdctr> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(5128).add(32 * n).cast()
            })
    }
    ///0x1408 - Channel 0 CANFD Control Register
    #[inline(always)]
    pub const fn cfdc0fdctr(&self) -> &Cfdcfdctr {
        self.cfdcfdctr(0)
    }
    ///0x1428 - Channel 1 CANFD Control Register
    #[inline(always)]
    pub const fn cfdc1fdctr(&self) -> &Cfdcfdctr {
        self.cfdcfdctr(1)
    }
    ///0x140c..0x1414 - Channel %s CANFD Status Register
    #[inline(always)]
    pub const fn cfdcfdsts(&self, n: usize) -> &Cfdcfdsts {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(5132).add(32 * n).cast() }
    }
    ///Iterator for array of:
    ///0x140c..0x1414 - Channel %s CANFD Status Register
    #[inline(always)]
    pub fn cfdcfdsts_iter(&self) -> impl Iterator<Item = &Cfdcfdsts> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(5132).add(32 * n).cast()
            })
    }
    ///0x140c - Channel 0 CANFD Status Register
    #[inline(always)]
    pub const fn cfdc0fdsts(&self) -> &Cfdcfdsts {
        self.cfdcfdsts(0)
    }
    ///0x142c - Channel 1 CANFD Status Register
    #[inline(always)]
    pub const fn cfdc1fdsts(&self) -> &Cfdcfdsts {
        self.cfdcfdsts(1)
    }
    ///0x1410..0x1418 - Channel %s CANFD CRC Register
    #[inline(always)]
    pub const fn cfdcfdcrc(&self, n: usize) -> &Cfdcfdcrc {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(5136).add(32 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1410..0x1418 - Channel %s CANFD CRC Register
    #[inline(always)]
    pub fn cfdcfdcrc_iter(&self) -> impl Iterator<Item = &Cfdcfdcrc> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(5136).add(32 * n).cast()
            })
    }
    ///0x1410 - Channel 0 CANFD CRC Register
    #[inline(always)]
    pub const fn cfdc0fdcrc(&self) -> &Cfdcfdcrc {
        self.cfdcfdcrc(0)
    }
    ///0x1430 - Channel 1 CANFD CRC Register
    #[inline(always)]
    pub const fn cfdc1fdcrc(&self) -> &Cfdcfdcrc {
        self.cfdcfdcrc(1)
    }
    ///0x1418..0x1420 - Channel %s Bus Load Control Register
    #[inline(always)]
    pub const fn cfdcblct(&self, n: usize) -> &Cfdcblct {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(5144).add(32 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1418..0x1420 - Channel %s Bus Load Control Register
    #[inline(always)]
    pub fn cfdcblct_iter(&self) -> impl Iterator<Item = &Cfdcblct> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(5144).add(32 * n).cast()
            })
    }
    ///0x1418 - Channel 0 Bus Load Control Register
    #[inline(always)]
    pub const fn cfdc0blct(&self) -> &Cfdcblct {
        self.cfdcblct(0)
    }
    ///0x1438 - Channel 1 Bus Load Control Register
    #[inline(always)]
    pub const fn cfdc1blct(&self) -> &Cfdcblct {
        self.cfdcblct(1)
    }
    ///0x141c..0x1424 - Channel %s Bus Load Status Register
    #[inline(always)]
    pub const fn cfdcblsts(&self, n: usize) -> &Cfdcblsts {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(5148).add(32 * n).cast() }
    }
    ///Iterator for array of:
    ///0x141c..0x1424 - Channel %s Bus Load Status Register
    #[inline(always)]
    pub fn cfdcblsts_iter(&self) -> impl Iterator<Item = &Cfdcblsts> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(5148).add(32 * n).cast()
            })
    }
    ///0x141c - Channel 0 Bus Load Status Register
    #[inline(always)]
    pub const fn cfdc0blsts(&self) -> &Cfdcblsts {
        self.cfdcblsts(0)
    }
    ///0x143c - Channel 1 Bus Load Status Register
    #[inline(always)]
    pub const fn cfdc1blsts(&self) -> &Cfdcblsts {
        self.cfdcblsts(1)
    }
    ///0x1800..0x1840 - Global Acceptance Filter List ID Registers
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CFDGAFLID1` register.</div>
    #[inline(always)]
    pub const fn cfdgaflid(&self, n: usize) -> &Cfdgaflid {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6144).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1800..0x1840 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub fn cfdgaflid_iter(&self) -> impl Iterator<Item = &Cfdgaflid> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(6144).add(16 * n).cast()
            })
    }
    ///0x1800 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid1(&self) -> &Cfdgaflid {
        self.cfdgaflid(0)
    }
    ///0x1810 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid2(&self) -> &Cfdgaflid {
        self.cfdgaflid(1)
    }
    ///0x1820 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid3(&self) -> &Cfdgaflid {
        self.cfdgaflid(2)
    }
    ///0x1830 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid4(&self) -> &Cfdgaflid {
        self.cfdgaflid(3)
    }
    ///0x1840 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid5(&self) -> &Cfdgaflid {
        self.cfdgaflid(4)
    }
    ///0x1850 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid6(&self) -> &Cfdgaflid {
        self.cfdgaflid(5)
    }
    ///0x1860 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid7(&self) -> &Cfdgaflid {
        self.cfdgaflid(6)
    }
    ///0x1870 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid8(&self) -> &Cfdgaflid {
        self.cfdgaflid(7)
    }
    ///0x1880 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid9(&self) -> &Cfdgaflid {
        self.cfdgaflid(8)
    }
    ///0x1890 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid10(&self) -> &Cfdgaflid {
        self.cfdgaflid(9)
    }
    ///0x18a0 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid11(&self) -> &Cfdgaflid {
        self.cfdgaflid(10)
    }
    ///0x18b0 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid12(&self) -> &Cfdgaflid {
        self.cfdgaflid(11)
    }
    ///0x18c0 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid13(&self) -> &Cfdgaflid {
        self.cfdgaflid(12)
    }
    ///0x18d0 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid14(&self) -> &Cfdgaflid {
        self.cfdgaflid(13)
    }
    ///0x18e0 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid15(&self) -> &Cfdgaflid {
        self.cfdgaflid(14)
    }
    ///0x18f0 - Global Acceptance Filter List ID Registers
    #[inline(always)]
    pub const fn cfdgaflid16(&self) -> &Cfdgaflid {
        self.cfdgaflid(15)
    }
    ///0x1804..0x1844 - Global Acceptance Filter List Mask Registers
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CFDGAFLM1` register.</div>
    #[inline(always)]
    pub const fn cfdgaflm(&self, n: usize) -> &Cfdgaflm {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6148).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1804..0x1844 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub fn cfdgaflm_iter(&self) -> impl Iterator<Item = &Cfdgaflm> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(6148).add(16 * n).cast()
            })
    }
    ///0x1804 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm1(&self) -> &Cfdgaflm {
        self.cfdgaflm(0)
    }
    ///0x1814 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm2(&self) -> &Cfdgaflm {
        self.cfdgaflm(1)
    }
    ///0x1824 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm3(&self) -> &Cfdgaflm {
        self.cfdgaflm(2)
    }
    ///0x1834 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm4(&self) -> &Cfdgaflm {
        self.cfdgaflm(3)
    }
    ///0x1844 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm5(&self) -> &Cfdgaflm {
        self.cfdgaflm(4)
    }
    ///0x1854 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm6(&self) -> &Cfdgaflm {
        self.cfdgaflm(5)
    }
    ///0x1864 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm7(&self) -> &Cfdgaflm {
        self.cfdgaflm(6)
    }
    ///0x1874 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm8(&self) -> &Cfdgaflm {
        self.cfdgaflm(7)
    }
    ///0x1884 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm9(&self) -> &Cfdgaflm {
        self.cfdgaflm(8)
    }
    ///0x1894 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm10(&self) -> &Cfdgaflm {
        self.cfdgaflm(9)
    }
    ///0x18a4 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm11(&self) -> &Cfdgaflm {
        self.cfdgaflm(10)
    }
    ///0x18b4 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm12(&self) -> &Cfdgaflm {
        self.cfdgaflm(11)
    }
    ///0x18c4 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm13(&self) -> &Cfdgaflm {
        self.cfdgaflm(12)
    }
    ///0x18d4 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm14(&self) -> &Cfdgaflm {
        self.cfdgaflm(13)
    }
    ///0x18e4 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm15(&self) -> &Cfdgaflm {
        self.cfdgaflm(14)
    }
    ///0x18f4 - Global Acceptance Filter List Mask Registers
    #[inline(always)]
    pub const fn cfdgaflm16(&self) -> &Cfdgaflm {
        self.cfdgaflm(15)
    }
    ///0x1808..0x1848 - Global Acceptance Filter List Pointer 0 Registers
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CFDGAFLP01` register.</div>
    #[inline(always)]
    pub const fn cfdgaflp0(&self, n: usize) -> &Cfdgaflp0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6152).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1808..0x1848 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub fn cfdgaflp0_iter(&self) -> impl Iterator<Item = &Cfdgaflp0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(6152).add(16 * n).cast()
            })
    }
    ///0x1808 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp01(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(0)
    }
    ///0x1818 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp02(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(1)
    }
    ///0x1828 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp03(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(2)
    }
    ///0x1838 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp04(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(3)
    }
    ///0x1848 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp05(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(4)
    }
    ///0x1858 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp06(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(5)
    }
    ///0x1868 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp07(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(6)
    }
    ///0x1878 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp08(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(7)
    }
    ///0x1888 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp09(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(8)
    }
    ///0x1898 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp010(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(9)
    }
    ///0x18a8 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp011(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(10)
    }
    ///0x18b8 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp012(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(11)
    }
    ///0x18c8 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp013(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(12)
    }
    ///0x18d8 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp014(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(13)
    }
    ///0x18e8 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp015(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(14)
    }
    ///0x18f8 - Global Acceptance Filter List Pointer 0 Registers
    #[inline(always)]
    pub const fn cfdgaflp016(&self) -> &Cfdgaflp0 {
        self.cfdgaflp0(15)
    }
    ///0x180c..0x184c - Global Acceptance Filter List Pointer 1 Registers
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CFDGAFLP11` register.</div>
    #[inline(always)]
    pub const fn cfdgaflp1(&self, n: usize) -> &Cfdgaflp1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6156).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x180c..0x184c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub fn cfdgaflp1_iter(&self) -> impl Iterator<Item = &Cfdgaflp1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(6156).add(16 * n).cast()
            })
    }
    ///0x180c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp11(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(0)
    }
    ///0x181c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp12(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(1)
    }
    ///0x182c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp13(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(2)
    }
    ///0x183c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp14(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(3)
    }
    ///0x184c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp15(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(4)
    }
    ///0x185c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp16(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(5)
    }
    ///0x186c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp17(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(6)
    }
    ///0x187c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp18(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(7)
    }
    ///0x188c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp19(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(8)
    }
    ///0x189c - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp110(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(9)
    }
    ///0x18ac - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp111(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(10)
    }
    ///0x18bc - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp112(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(11)
    }
    ///0x18cc - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp113(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(12)
    }
    ///0x18dc - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp114(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(13)
    }
    ///0x18ec - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp115(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(14)
    }
    ///0x18fc - Global Acceptance Filter List Pointer 1 Registers
    #[inline(always)]
    pub const fn cfdgaflp116(&self) -> &Cfdgaflp1 {
        self.cfdgaflp1(15)
    }
    ///0x2000..0x2040 - RX Message Buffer ID Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmid_0(&self, n: usize) -> &Cfdrmid0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8192).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2000..0x2040 - RX Message Buffer ID Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmid_0_iter(&self) -> impl Iterator<Item = &Cfdrmid0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8192).add(128 * n).cast()
            })
    }
    ///0x2000 - RX Message Buffer ID Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmid0_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(0)
    }
    ///0x2080 - RX Message Buffer ID Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmid1_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(1)
    }
    ///0x2100 - RX Message Buffer ID Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmid2_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(2)
    }
    ///0x2180 - RX Message Buffer ID Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmid3_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(3)
    }
    ///0x2200 - RX Message Buffer ID Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmid4_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(4)
    }
    ///0x2280 - RX Message Buffer ID Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmid5_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(5)
    }
    ///0x2300 - RX Message Buffer ID Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmid6_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(6)
    }
    ///0x2380 - RX Message Buffer ID Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmid7_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(7)
    }
    ///0x2400 - RX Message Buffer ID Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmid8_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(8)
    }
    ///0x2480 - RX Message Buffer ID Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmid9_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(9)
    }
    ///0x2500 - RX Message Buffer ID Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmid10_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(10)
    }
    ///0x2580 - RX Message Buffer ID Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmid11_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(11)
    }
    ///0x2600 - RX Message Buffer ID Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmid12_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(12)
    }
    ///0x2680 - RX Message Buffer ID Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmid13_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(13)
    }
    ///0x2700 - RX Message Buffer ID Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmid14_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(14)
    }
    ///0x2780 - RX Message Buffer ID Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmid15_0(&self) -> &Cfdrmid0 {
        self.cfdrmid_0(15)
    }
    ///0x2004..0x2044 - RX Message Buffer Pointer Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmptr_0(&self, n: usize) -> &Cfdrmptr0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8196).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2004..0x2044 - RX Message Buffer Pointer Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmptr_0_iter(&self) -> impl Iterator<Item = &Cfdrmptr0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8196).add(128 * n).cast()
            })
    }
    ///0x2004 - RX Message Buffer Pointer Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr0_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(0)
    }
    ///0x2084 - RX Message Buffer Pointer Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr1_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(1)
    }
    ///0x2104 - RX Message Buffer Pointer Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr2_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(2)
    }
    ///0x2184 - RX Message Buffer Pointer Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr3_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(3)
    }
    ///0x2204 - RX Message Buffer Pointer Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr4_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(4)
    }
    ///0x2284 - RX Message Buffer Pointer Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr5_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(5)
    }
    ///0x2304 - RX Message Buffer Pointer Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr6_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(6)
    }
    ///0x2384 - RX Message Buffer Pointer Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr7_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(7)
    }
    ///0x2404 - RX Message Buffer Pointer Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr8_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(8)
    }
    ///0x2484 - RX Message Buffer Pointer Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr9_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(9)
    }
    ///0x2504 - RX Message Buffer Pointer Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr10_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(10)
    }
    ///0x2584 - RX Message Buffer Pointer Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr11_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(11)
    }
    ///0x2604 - RX Message Buffer Pointer Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr12_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(12)
    }
    ///0x2684 - RX Message Buffer Pointer Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr13_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(13)
    }
    ///0x2704 - RX Message Buffer Pointer Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr14_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(14)
    }
    ///0x2784 - RX Message Buffer Pointer Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmptr15_0(&self) -> &Cfdrmptr0 {
        self.cfdrmptr_0(15)
    }
    ///0x2008..0x2048 - RX Message Buffer CAN-FD Status Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts_0(&self, n: usize) -> &Cfdrmfdsts0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8200).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2008..0x2048 - RX Message Buffer CAN-FD Status Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmfdsts_0_iter(&self) -> impl Iterator<Item = &Cfdrmfdsts0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8200).add(128 * n).cast()
            })
    }
    ///0x2008 - RX Message Buffer CAN-FD Status Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts0_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(0)
    }
    ///0x2088 - RX Message Buffer CAN-FD Status Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts1_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(1)
    }
    ///0x2108 - RX Message Buffer CAN-FD Status Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts2_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(2)
    }
    ///0x2188 - RX Message Buffer CAN-FD Status Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts3_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(3)
    }
    ///0x2208 - RX Message Buffer CAN-FD Status Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts4_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(4)
    }
    ///0x2288 - RX Message Buffer CAN-FD Status Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts5_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(5)
    }
    ///0x2308 - RX Message Buffer CAN-FD Status Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts6_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(6)
    }
    ///0x2388 - RX Message Buffer CAN-FD Status Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts7_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(7)
    }
    ///0x2408 - RX Message Buffer CAN-FD Status Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts8_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(8)
    }
    ///0x2488 - RX Message Buffer CAN-FD Status Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts9_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(9)
    }
    ///0x2508 - RX Message Buffer CAN-FD Status Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts10_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(10)
    }
    ///0x2588 - RX Message Buffer CAN-FD Status Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts11_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(11)
    }
    ///0x2608 - RX Message Buffer CAN-FD Status Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts12_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(12)
    }
    ///0x2688 - RX Message Buffer CAN-FD Status Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts13_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(13)
    }
    ///0x2708 - RX Message Buffer CAN-FD Status Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts14_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(14)
    }
    ///0x2788 - RX Message Buffer CAN-FD Status Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmfdsts15_0(&self) -> &Cfdrmfdsts0 {
        self.cfdrmfdsts_0(15)
    }
    ///0x200c..0x204c - RX Message Buffer Data Field 0 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0__0(&self, n: usize) -> &Cfdrmdf0_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8204).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x200c..0x204c - RX Message Buffer Data Field 0 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf0__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf0_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8204).add(128 * n).cast()
            })
    }
    ///0x200c - RX Message Buffer Data Field 0 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_0_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(0)
    }
    ///0x208c - RX Message Buffer Data Field 0 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_1_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(1)
    }
    ///0x210c - RX Message Buffer Data Field 0 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_2_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(2)
    }
    ///0x218c - RX Message Buffer Data Field 0 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_3_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(3)
    }
    ///0x220c - RX Message Buffer Data Field 0 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_4_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(4)
    }
    ///0x228c - RX Message Buffer Data Field 0 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_5_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(5)
    }
    ///0x230c - RX Message Buffer Data Field 0 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_6_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(6)
    }
    ///0x238c - RX Message Buffer Data Field 0 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_7_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(7)
    }
    ///0x240c - RX Message Buffer Data Field 0 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_8_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(8)
    }
    ///0x248c - RX Message Buffer Data Field 0 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_9_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(9)
    }
    ///0x250c - RX Message Buffer Data Field 0 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_10_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(10)
    }
    ///0x258c - RX Message Buffer Data Field 0 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_11_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(11)
    }
    ///0x260c - RX Message Buffer Data Field 0 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_12_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(12)
    }
    ///0x268c - RX Message Buffer Data Field 0 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_13_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(13)
    }
    ///0x270c - RX Message Buffer Data Field 0 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_14_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(14)
    }
    ///0x278c - RX Message Buffer Data Field 0 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf0_15_0(&self) -> &Cfdrmdf0_0 {
        self.cfdrmdf0__0(15)
    }
    ///0x2010..0x2050 - RX Message Buffer Data Field 1 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1__0(&self, n: usize) -> &Cfdrmdf1_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8208).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2010..0x2050 - RX Message Buffer Data Field 1 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf1__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf1_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8208).add(128 * n).cast()
            })
    }
    ///0x2010 - RX Message Buffer Data Field 1 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_0_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(0)
    }
    ///0x2090 - RX Message Buffer Data Field 1 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_1_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(1)
    }
    ///0x2110 - RX Message Buffer Data Field 1 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_2_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(2)
    }
    ///0x2190 - RX Message Buffer Data Field 1 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_3_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(3)
    }
    ///0x2210 - RX Message Buffer Data Field 1 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_4_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(4)
    }
    ///0x2290 - RX Message Buffer Data Field 1 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_5_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(5)
    }
    ///0x2310 - RX Message Buffer Data Field 1 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_6_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(6)
    }
    ///0x2390 - RX Message Buffer Data Field 1 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_7_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(7)
    }
    ///0x2410 - RX Message Buffer Data Field 1 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_8_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(8)
    }
    ///0x2490 - RX Message Buffer Data Field 1 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_9_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(9)
    }
    ///0x2510 - RX Message Buffer Data Field 1 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_10_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(10)
    }
    ///0x2590 - RX Message Buffer Data Field 1 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_11_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(11)
    }
    ///0x2610 - RX Message Buffer Data Field 1 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_12_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(12)
    }
    ///0x2690 - RX Message Buffer Data Field 1 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_13_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(13)
    }
    ///0x2710 - RX Message Buffer Data Field 1 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_14_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(14)
    }
    ///0x2790 - RX Message Buffer Data Field 1 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf1_15_0(&self) -> &Cfdrmdf1_0 {
        self.cfdrmdf1__0(15)
    }
    ///0x2014..0x2054 - RX Message Buffer Data Field 2 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2__0(&self, n: usize) -> &Cfdrmdf2_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8212).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2014..0x2054 - RX Message Buffer Data Field 2 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf2__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf2_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8212).add(128 * n).cast()
            })
    }
    ///0x2014 - RX Message Buffer Data Field 2 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_0_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(0)
    }
    ///0x2094 - RX Message Buffer Data Field 2 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_1_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(1)
    }
    ///0x2114 - RX Message Buffer Data Field 2 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_2_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(2)
    }
    ///0x2194 - RX Message Buffer Data Field 2 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_3_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(3)
    }
    ///0x2214 - RX Message Buffer Data Field 2 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_4_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(4)
    }
    ///0x2294 - RX Message Buffer Data Field 2 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_5_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(5)
    }
    ///0x2314 - RX Message Buffer Data Field 2 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_6_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(6)
    }
    ///0x2394 - RX Message Buffer Data Field 2 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_7_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(7)
    }
    ///0x2414 - RX Message Buffer Data Field 2 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_8_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(8)
    }
    ///0x2494 - RX Message Buffer Data Field 2 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_9_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(9)
    }
    ///0x2514 - RX Message Buffer Data Field 2 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_10_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(10)
    }
    ///0x2594 - RX Message Buffer Data Field 2 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_11_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(11)
    }
    ///0x2614 - RX Message Buffer Data Field 2 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_12_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(12)
    }
    ///0x2694 - RX Message Buffer Data Field 2 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_13_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(13)
    }
    ///0x2714 - RX Message Buffer Data Field 2 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_14_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(14)
    }
    ///0x2794 - RX Message Buffer Data Field 2 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf2_15_0(&self) -> &Cfdrmdf2_0 {
        self.cfdrmdf2__0(15)
    }
    ///0x2018..0x2058 - RX Message Buffer Data Field 3 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3__0(&self, n: usize) -> &Cfdrmdf3_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8216).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2018..0x2058 - RX Message Buffer Data Field 3 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf3__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf3_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8216).add(128 * n).cast()
            })
    }
    ///0x2018 - RX Message Buffer Data Field 3 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_0_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(0)
    }
    ///0x2098 - RX Message Buffer Data Field 3 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_1_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(1)
    }
    ///0x2118 - RX Message Buffer Data Field 3 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_2_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(2)
    }
    ///0x2198 - RX Message Buffer Data Field 3 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_3_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(3)
    }
    ///0x2218 - RX Message Buffer Data Field 3 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_4_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(4)
    }
    ///0x2298 - RX Message Buffer Data Field 3 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_5_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(5)
    }
    ///0x2318 - RX Message Buffer Data Field 3 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_6_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(6)
    }
    ///0x2398 - RX Message Buffer Data Field 3 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_7_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(7)
    }
    ///0x2418 - RX Message Buffer Data Field 3 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_8_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(8)
    }
    ///0x2498 - RX Message Buffer Data Field 3 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_9_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(9)
    }
    ///0x2518 - RX Message Buffer Data Field 3 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_10_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(10)
    }
    ///0x2598 - RX Message Buffer Data Field 3 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_11_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(11)
    }
    ///0x2618 - RX Message Buffer Data Field 3 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_12_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(12)
    }
    ///0x2698 - RX Message Buffer Data Field 3 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_13_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(13)
    }
    ///0x2718 - RX Message Buffer Data Field 3 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_14_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(14)
    }
    ///0x2798 - RX Message Buffer Data Field 3 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf3_15_0(&self) -> &Cfdrmdf3_0 {
        self.cfdrmdf3__0(15)
    }
    ///0x201c..0x205c - RX Message Buffer Data Field 4 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4__0(&self, n: usize) -> &Cfdrmdf4_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8220).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x201c..0x205c - RX Message Buffer Data Field 4 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf4__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf4_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8220).add(128 * n).cast()
            })
    }
    ///0x201c - RX Message Buffer Data Field 4 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_0_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(0)
    }
    ///0x209c - RX Message Buffer Data Field 4 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_1_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(1)
    }
    ///0x211c - RX Message Buffer Data Field 4 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_2_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(2)
    }
    ///0x219c - RX Message Buffer Data Field 4 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_3_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(3)
    }
    ///0x221c - RX Message Buffer Data Field 4 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_4_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(4)
    }
    ///0x229c - RX Message Buffer Data Field 4 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_5_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(5)
    }
    ///0x231c - RX Message Buffer Data Field 4 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_6_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(6)
    }
    ///0x239c - RX Message Buffer Data Field 4 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_7_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(7)
    }
    ///0x241c - RX Message Buffer Data Field 4 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_8_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(8)
    }
    ///0x249c - RX Message Buffer Data Field 4 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_9_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(9)
    }
    ///0x251c - RX Message Buffer Data Field 4 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_10_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(10)
    }
    ///0x259c - RX Message Buffer Data Field 4 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_11_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(11)
    }
    ///0x261c - RX Message Buffer Data Field 4 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_12_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(12)
    }
    ///0x269c - RX Message Buffer Data Field 4 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_13_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(13)
    }
    ///0x271c - RX Message Buffer Data Field 4 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_14_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(14)
    }
    ///0x279c - RX Message Buffer Data Field 4 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf4_15_0(&self) -> &Cfdrmdf4_0 {
        self.cfdrmdf4__0(15)
    }
    ///0x2020..0x2060 - RX Message Buffer Data Field 5 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5__0(&self, n: usize) -> &Cfdrmdf5_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8224).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2020..0x2060 - RX Message Buffer Data Field 5 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf5__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf5_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8224).add(128 * n).cast()
            })
    }
    ///0x2020 - RX Message Buffer Data Field 5 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_0_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(0)
    }
    ///0x20a0 - RX Message Buffer Data Field 5 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_1_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(1)
    }
    ///0x2120 - RX Message Buffer Data Field 5 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_2_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(2)
    }
    ///0x21a0 - RX Message Buffer Data Field 5 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_3_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(3)
    }
    ///0x2220 - RX Message Buffer Data Field 5 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_4_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(4)
    }
    ///0x22a0 - RX Message Buffer Data Field 5 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_5_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(5)
    }
    ///0x2320 - RX Message Buffer Data Field 5 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_6_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(6)
    }
    ///0x23a0 - RX Message Buffer Data Field 5 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_7_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(7)
    }
    ///0x2420 - RX Message Buffer Data Field 5 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_8_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(8)
    }
    ///0x24a0 - RX Message Buffer Data Field 5 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_9_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(9)
    }
    ///0x2520 - RX Message Buffer Data Field 5 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_10_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(10)
    }
    ///0x25a0 - RX Message Buffer Data Field 5 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_11_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(11)
    }
    ///0x2620 - RX Message Buffer Data Field 5 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_12_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(12)
    }
    ///0x26a0 - RX Message Buffer Data Field 5 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_13_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(13)
    }
    ///0x2720 - RX Message Buffer Data Field 5 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_14_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(14)
    }
    ///0x27a0 - RX Message Buffer Data Field 5 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf5_15_0(&self) -> &Cfdrmdf5_0 {
        self.cfdrmdf5__0(15)
    }
    ///0x2024..0x2064 - RX Message Buffer Data Field 6 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6__0(&self, n: usize) -> &Cfdrmdf6_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8228).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2024..0x2064 - RX Message Buffer Data Field 6 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf6__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf6_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8228).add(128 * n).cast()
            })
    }
    ///0x2024 - RX Message Buffer Data Field 6 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_0_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(0)
    }
    ///0x20a4 - RX Message Buffer Data Field 6 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_1_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(1)
    }
    ///0x2124 - RX Message Buffer Data Field 6 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_2_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(2)
    }
    ///0x21a4 - RX Message Buffer Data Field 6 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_3_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(3)
    }
    ///0x2224 - RX Message Buffer Data Field 6 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_4_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(4)
    }
    ///0x22a4 - RX Message Buffer Data Field 6 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_5_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(5)
    }
    ///0x2324 - RX Message Buffer Data Field 6 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_6_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(6)
    }
    ///0x23a4 - RX Message Buffer Data Field 6 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_7_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(7)
    }
    ///0x2424 - RX Message Buffer Data Field 6 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_8_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(8)
    }
    ///0x24a4 - RX Message Buffer Data Field 6 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_9_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(9)
    }
    ///0x2524 - RX Message Buffer Data Field 6 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_10_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(10)
    }
    ///0x25a4 - RX Message Buffer Data Field 6 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_11_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(11)
    }
    ///0x2624 - RX Message Buffer Data Field 6 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_12_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(12)
    }
    ///0x26a4 - RX Message Buffer Data Field 6 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_13_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(13)
    }
    ///0x2724 - RX Message Buffer Data Field 6 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_14_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(14)
    }
    ///0x27a4 - RX Message Buffer Data Field 6 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf6_15_0(&self) -> &Cfdrmdf6_0 {
        self.cfdrmdf6__0(15)
    }
    ///0x2028..0x2068 - RX Message Buffer Data Field 7 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7__0(&self, n: usize) -> &Cfdrmdf7_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8232).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2028..0x2068 - RX Message Buffer Data Field 7 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf7__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf7_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8232).add(128 * n).cast()
            })
    }
    ///0x2028 - RX Message Buffer Data Field 7 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_0_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(0)
    }
    ///0x20a8 - RX Message Buffer Data Field 7 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_1_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(1)
    }
    ///0x2128 - RX Message Buffer Data Field 7 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_2_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(2)
    }
    ///0x21a8 - RX Message Buffer Data Field 7 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_3_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(3)
    }
    ///0x2228 - RX Message Buffer Data Field 7 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_4_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(4)
    }
    ///0x22a8 - RX Message Buffer Data Field 7 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_5_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(5)
    }
    ///0x2328 - RX Message Buffer Data Field 7 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_6_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(6)
    }
    ///0x23a8 - RX Message Buffer Data Field 7 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_7_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(7)
    }
    ///0x2428 - RX Message Buffer Data Field 7 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_8_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(8)
    }
    ///0x24a8 - RX Message Buffer Data Field 7 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_9_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(9)
    }
    ///0x2528 - RX Message Buffer Data Field 7 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_10_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(10)
    }
    ///0x25a8 - RX Message Buffer Data Field 7 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_11_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(11)
    }
    ///0x2628 - RX Message Buffer Data Field 7 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_12_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(12)
    }
    ///0x26a8 - RX Message Buffer Data Field 7 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_13_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(13)
    }
    ///0x2728 - RX Message Buffer Data Field 7 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_14_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(14)
    }
    ///0x27a8 - RX Message Buffer Data Field 7 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf7_15_0(&self) -> &Cfdrmdf7_0 {
        self.cfdrmdf7__0(15)
    }
    ///0x202c..0x206c - RX Message Buffer Data Field 8 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8__0(&self, n: usize) -> &Cfdrmdf8_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8236).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x202c..0x206c - RX Message Buffer Data Field 8 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf8__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf8_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8236).add(128 * n).cast()
            })
    }
    ///0x202c - RX Message Buffer Data Field 8 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_0_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(0)
    }
    ///0x20ac - RX Message Buffer Data Field 8 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_1_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(1)
    }
    ///0x212c - RX Message Buffer Data Field 8 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_2_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(2)
    }
    ///0x21ac - RX Message Buffer Data Field 8 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_3_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(3)
    }
    ///0x222c - RX Message Buffer Data Field 8 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_4_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(4)
    }
    ///0x22ac - RX Message Buffer Data Field 8 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_5_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(5)
    }
    ///0x232c - RX Message Buffer Data Field 8 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_6_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(6)
    }
    ///0x23ac - RX Message Buffer Data Field 8 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_7_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(7)
    }
    ///0x242c - RX Message Buffer Data Field 8 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_8_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(8)
    }
    ///0x24ac - RX Message Buffer Data Field 8 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_9_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(9)
    }
    ///0x252c - RX Message Buffer Data Field 8 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_10_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(10)
    }
    ///0x25ac - RX Message Buffer Data Field 8 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_11_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(11)
    }
    ///0x262c - RX Message Buffer Data Field 8 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_12_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(12)
    }
    ///0x26ac - RX Message Buffer Data Field 8 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_13_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(13)
    }
    ///0x272c - RX Message Buffer Data Field 8 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_14_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(14)
    }
    ///0x27ac - RX Message Buffer Data Field 8 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf8_15_0(&self) -> &Cfdrmdf8_0 {
        self.cfdrmdf8__0(15)
    }
    ///0x2030..0x2070 - RX Message Buffer Data Field 9 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9__0(&self, n: usize) -> &Cfdrmdf9_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8240).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2030..0x2070 - RX Message Buffer Data Field 9 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf9__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf9_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8240).add(128 * n).cast()
            })
    }
    ///0x2030 - RX Message Buffer Data Field 9 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_0_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(0)
    }
    ///0x20b0 - RX Message Buffer Data Field 9 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_1_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(1)
    }
    ///0x2130 - RX Message Buffer Data Field 9 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_2_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(2)
    }
    ///0x21b0 - RX Message Buffer Data Field 9 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_3_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(3)
    }
    ///0x2230 - RX Message Buffer Data Field 9 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_4_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(4)
    }
    ///0x22b0 - RX Message Buffer Data Field 9 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_5_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(5)
    }
    ///0x2330 - RX Message Buffer Data Field 9 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_6_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(6)
    }
    ///0x23b0 - RX Message Buffer Data Field 9 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_7_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(7)
    }
    ///0x2430 - RX Message Buffer Data Field 9 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_8_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(8)
    }
    ///0x24b0 - RX Message Buffer Data Field 9 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_9_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(9)
    }
    ///0x2530 - RX Message Buffer Data Field 9 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_10_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(10)
    }
    ///0x25b0 - RX Message Buffer Data Field 9 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_11_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(11)
    }
    ///0x2630 - RX Message Buffer Data Field 9 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_12_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(12)
    }
    ///0x26b0 - RX Message Buffer Data Field 9 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_13_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(13)
    }
    ///0x2730 - RX Message Buffer Data Field 9 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_14_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(14)
    }
    ///0x27b0 - RX Message Buffer Data Field 9 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf9_15_0(&self) -> &Cfdrmdf9_0 {
        self.cfdrmdf9__0(15)
    }
    ///0x2034..0x2074 - RX Message Buffer Data Field 10 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10__0(&self, n: usize) -> &Cfdrmdf10_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8244).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2034..0x2074 - RX Message Buffer Data Field 10 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf10__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf10_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8244).add(128 * n).cast()
            })
    }
    ///0x2034 - RX Message Buffer Data Field 10 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_0_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(0)
    }
    ///0x20b4 - RX Message Buffer Data Field 10 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_1_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(1)
    }
    ///0x2134 - RX Message Buffer Data Field 10 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_2_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(2)
    }
    ///0x21b4 - RX Message Buffer Data Field 10 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_3_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(3)
    }
    ///0x2234 - RX Message Buffer Data Field 10 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_4_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(4)
    }
    ///0x22b4 - RX Message Buffer Data Field 10 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_5_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(5)
    }
    ///0x2334 - RX Message Buffer Data Field 10 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_6_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(6)
    }
    ///0x23b4 - RX Message Buffer Data Field 10 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_7_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(7)
    }
    ///0x2434 - RX Message Buffer Data Field 10 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_8_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(8)
    }
    ///0x24b4 - RX Message Buffer Data Field 10 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_9_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(9)
    }
    ///0x2534 - RX Message Buffer Data Field 10 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_10_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(10)
    }
    ///0x25b4 - RX Message Buffer Data Field 10 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_11_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(11)
    }
    ///0x2634 - RX Message Buffer Data Field 10 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_12_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(12)
    }
    ///0x26b4 - RX Message Buffer Data Field 10 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_13_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(13)
    }
    ///0x2734 - RX Message Buffer Data Field 10 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_14_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(14)
    }
    ///0x27b4 - RX Message Buffer Data Field 10 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf10_15_0(&self) -> &Cfdrmdf10_0 {
        self.cfdrmdf10__0(15)
    }
    ///0x2038..0x2078 - RX Message Buffer Data Field 11 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11__0(&self, n: usize) -> &Cfdrmdf11_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8248).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2038..0x2078 - RX Message Buffer Data Field 11 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf11__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf11_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8248).add(128 * n).cast()
            })
    }
    ///0x2038 - RX Message Buffer Data Field 11 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_0_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(0)
    }
    ///0x20b8 - RX Message Buffer Data Field 11 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_1_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(1)
    }
    ///0x2138 - RX Message Buffer Data Field 11 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_2_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(2)
    }
    ///0x21b8 - RX Message Buffer Data Field 11 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_3_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(3)
    }
    ///0x2238 - RX Message Buffer Data Field 11 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_4_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(4)
    }
    ///0x22b8 - RX Message Buffer Data Field 11 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_5_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(5)
    }
    ///0x2338 - RX Message Buffer Data Field 11 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_6_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(6)
    }
    ///0x23b8 - RX Message Buffer Data Field 11 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_7_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(7)
    }
    ///0x2438 - RX Message Buffer Data Field 11 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_8_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(8)
    }
    ///0x24b8 - RX Message Buffer Data Field 11 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_9_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(9)
    }
    ///0x2538 - RX Message Buffer Data Field 11 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_10_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(10)
    }
    ///0x25b8 - RX Message Buffer Data Field 11 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_11_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(11)
    }
    ///0x2638 - RX Message Buffer Data Field 11 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_12_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(12)
    }
    ///0x26b8 - RX Message Buffer Data Field 11 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_13_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(13)
    }
    ///0x2738 - RX Message Buffer Data Field 11 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_14_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(14)
    }
    ///0x27b8 - RX Message Buffer Data Field 11 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf11_15_0(&self) -> &Cfdrmdf11_0 {
        self.cfdrmdf11__0(15)
    }
    ///0x203c..0x207c - RX Message Buffer Data Field 12 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12__0(&self, n: usize) -> &Cfdrmdf12_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8252).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x203c..0x207c - RX Message Buffer Data Field 12 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf12__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf12_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8252).add(128 * n).cast()
            })
    }
    ///0x203c - RX Message Buffer Data Field 12 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_0_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(0)
    }
    ///0x20bc - RX Message Buffer Data Field 12 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_1_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(1)
    }
    ///0x213c - RX Message Buffer Data Field 12 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_2_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(2)
    }
    ///0x21bc - RX Message Buffer Data Field 12 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_3_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(3)
    }
    ///0x223c - RX Message Buffer Data Field 12 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_4_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(4)
    }
    ///0x22bc - RX Message Buffer Data Field 12 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_5_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(5)
    }
    ///0x233c - RX Message Buffer Data Field 12 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_6_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(6)
    }
    ///0x23bc - RX Message Buffer Data Field 12 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_7_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(7)
    }
    ///0x243c - RX Message Buffer Data Field 12 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_8_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(8)
    }
    ///0x24bc - RX Message Buffer Data Field 12 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_9_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(9)
    }
    ///0x253c - RX Message Buffer Data Field 12 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_10_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(10)
    }
    ///0x25bc - RX Message Buffer Data Field 12 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_11_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(11)
    }
    ///0x263c - RX Message Buffer Data Field 12 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_12_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(12)
    }
    ///0x26bc - RX Message Buffer Data Field 12 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_13_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(13)
    }
    ///0x273c - RX Message Buffer Data Field 12 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_14_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(14)
    }
    ///0x27bc - RX Message Buffer Data Field 12 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf12_15_0(&self) -> &Cfdrmdf12_0 {
        self.cfdrmdf12__0(15)
    }
    ///0x2040..0x2080 - RX Message Buffer Data Field 13 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13__0(&self, n: usize) -> &Cfdrmdf13_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8256).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2040..0x2080 - RX Message Buffer Data Field 13 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf13__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf13_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8256).add(128 * n).cast()
            })
    }
    ///0x2040 - RX Message Buffer Data Field 13 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_0_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(0)
    }
    ///0x20c0 - RX Message Buffer Data Field 13 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_1_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(1)
    }
    ///0x2140 - RX Message Buffer Data Field 13 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_2_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(2)
    }
    ///0x21c0 - RX Message Buffer Data Field 13 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_3_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(3)
    }
    ///0x2240 - RX Message Buffer Data Field 13 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_4_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(4)
    }
    ///0x22c0 - RX Message Buffer Data Field 13 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_5_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(5)
    }
    ///0x2340 - RX Message Buffer Data Field 13 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_6_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(6)
    }
    ///0x23c0 - RX Message Buffer Data Field 13 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_7_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(7)
    }
    ///0x2440 - RX Message Buffer Data Field 13 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_8_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(8)
    }
    ///0x24c0 - RX Message Buffer Data Field 13 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_9_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(9)
    }
    ///0x2540 - RX Message Buffer Data Field 13 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_10_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(10)
    }
    ///0x25c0 - RX Message Buffer Data Field 13 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_11_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(11)
    }
    ///0x2640 - RX Message Buffer Data Field 13 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_12_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(12)
    }
    ///0x26c0 - RX Message Buffer Data Field 13 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_13_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(13)
    }
    ///0x2740 - RX Message Buffer Data Field 13 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_14_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(14)
    }
    ///0x27c0 - RX Message Buffer Data Field 13 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf13_15_0(&self) -> &Cfdrmdf13_0 {
        self.cfdrmdf13__0(15)
    }
    ///0x2044..0x2084 - RX Message Buffer Data Field 14 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14__0(&self, n: usize) -> &Cfdrmdf14_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8260).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2044..0x2084 - RX Message Buffer Data Field 14 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf14__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf14_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8260).add(128 * n).cast()
            })
    }
    ///0x2044 - RX Message Buffer Data Field 14 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_0_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(0)
    }
    ///0x20c4 - RX Message Buffer Data Field 14 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_1_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(1)
    }
    ///0x2144 - RX Message Buffer Data Field 14 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_2_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(2)
    }
    ///0x21c4 - RX Message Buffer Data Field 14 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_3_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(3)
    }
    ///0x2244 - RX Message Buffer Data Field 14 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_4_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(4)
    }
    ///0x22c4 - RX Message Buffer Data Field 14 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_5_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(5)
    }
    ///0x2344 - RX Message Buffer Data Field 14 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_6_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(6)
    }
    ///0x23c4 - RX Message Buffer Data Field 14 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_7_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(7)
    }
    ///0x2444 - RX Message Buffer Data Field 14 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_8_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(8)
    }
    ///0x24c4 - RX Message Buffer Data Field 14 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_9_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(9)
    }
    ///0x2544 - RX Message Buffer Data Field 14 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_10_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(10)
    }
    ///0x25c4 - RX Message Buffer Data Field 14 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_11_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(11)
    }
    ///0x2644 - RX Message Buffer Data Field 14 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_12_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(12)
    }
    ///0x26c4 - RX Message Buffer Data Field 14 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_13_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(13)
    }
    ///0x2744 - RX Message Buffer Data Field 14 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_14_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(14)
    }
    ///0x27c4 - RX Message Buffer Data Field 14 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf14_15_0(&self) -> &Cfdrmdf14_0 {
        self.cfdrmdf14__0(15)
    }
    ///0x2048..0x2088 - RX Message Buffer Data Field 15 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15__0(&self, n: usize) -> &Cfdrmdf15_0 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8264).add(128 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2048..0x2088 - RX Message Buffer Data Field 15 Register %s Channel 0
    #[inline(always)]
    pub fn cfdrmdf15__0_iter(&self) -> impl Iterator<Item = &Cfdrmdf15_0> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8264).add(128 * n).cast()
            })
    }
    ///0x2048 - RX Message Buffer Data Field 15 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_0_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(0)
    }
    ///0x20c8 - RX Message Buffer Data Field 15 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_1_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(1)
    }
    ///0x2148 - RX Message Buffer Data Field 15 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_2_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(2)
    }
    ///0x21c8 - RX Message Buffer Data Field 15 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_3_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(3)
    }
    ///0x2248 - RX Message Buffer Data Field 15 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_4_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(4)
    }
    ///0x22c8 - RX Message Buffer Data Field 15 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_5_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(5)
    }
    ///0x2348 - RX Message Buffer Data Field 15 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_6_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(6)
    }
    ///0x23c8 - RX Message Buffer Data Field 15 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_7_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(7)
    }
    ///0x2448 - RX Message Buffer Data Field 15 Register 8 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_8_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(8)
    }
    ///0x24c8 - RX Message Buffer Data Field 15 Register 9 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_9_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(9)
    }
    ///0x2548 - RX Message Buffer Data Field 15 Register 10 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_10_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(10)
    }
    ///0x25c8 - RX Message Buffer Data Field 15 Register 11 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_11_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(11)
    }
    ///0x2648 - RX Message Buffer Data Field 15 Register 12 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_12_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(12)
    }
    ///0x26c8 - RX Message Buffer Data Field 15 Register 13 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_13_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(13)
    }
    ///0x2748 - RX Message Buffer Data Field 15 Register 14 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_14_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(14)
    }
    ///0x27c8 - RX Message Buffer Data Field 15 Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdrmdf15_15_0(&self) -> &Cfdrmdf15_0 {
        self.cfdrmdf15__0(15)
    }
    ///0x2800..0x2840 - RX Message Buffer ID Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmid_1(&self, n: usize) -> &Cfdrmid1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10240).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2800..0x2840 - RX Message Buffer ID Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmid_1_iter(&self) -> impl Iterator<Item = &Cfdrmid1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10240).add(128 * n).cast()
            })
    }
    ///0x2800 - RX Message Buffer ID Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmid0_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(0)
    }
    ///0x2880 - RX Message Buffer ID Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmid1_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(1)
    }
    ///0x2900 - RX Message Buffer ID Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmid2_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(2)
    }
    ///0x2980 - RX Message Buffer ID Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmid3_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(3)
    }
    ///0x2a00 - RX Message Buffer ID Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmid4_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(4)
    }
    ///0x2a80 - RX Message Buffer ID Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmid5_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(5)
    }
    ///0x2b00 - RX Message Buffer ID Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmid6_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(6)
    }
    ///0x2b80 - RX Message Buffer ID Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmid7_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(7)
    }
    ///0x2c00 - RX Message Buffer ID Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmid8_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(8)
    }
    ///0x2c80 - RX Message Buffer ID Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmid9_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(9)
    }
    ///0x2d00 - RX Message Buffer ID Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmid10_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(10)
    }
    ///0x2d80 - RX Message Buffer ID Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmid11_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(11)
    }
    ///0x2e00 - RX Message Buffer ID Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmid12_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(12)
    }
    ///0x2e80 - RX Message Buffer ID Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmid13_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(13)
    }
    ///0x2f00 - RX Message Buffer ID Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmid14_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(14)
    }
    ///0x2f80 - RX Message Buffer ID Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmid15_1(&self) -> &Cfdrmid1 {
        self.cfdrmid_1(15)
    }
    ///0x2804..0x2844 - RX Message Buffer Pointer Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmptr_1(&self, n: usize) -> &Cfdrmptr1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10244).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2804..0x2844 - RX Message Buffer Pointer Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmptr_1_iter(&self) -> impl Iterator<Item = &Cfdrmptr1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10244).add(128 * n).cast()
            })
    }
    ///0x2804 - RX Message Buffer Pointer Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr0_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(0)
    }
    ///0x2884 - RX Message Buffer Pointer Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr1_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(1)
    }
    ///0x2904 - RX Message Buffer Pointer Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr2_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(2)
    }
    ///0x2984 - RX Message Buffer Pointer Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr3_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(3)
    }
    ///0x2a04 - RX Message Buffer Pointer Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr4_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(4)
    }
    ///0x2a84 - RX Message Buffer Pointer Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr5_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(5)
    }
    ///0x2b04 - RX Message Buffer Pointer Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr6_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(6)
    }
    ///0x2b84 - RX Message Buffer Pointer Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr7_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(7)
    }
    ///0x2c04 - RX Message Buffer Pointer Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr8_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(8)
    }
    ///0x2c84 - RX Message Buffer Pointer Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr9_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(9)
    }
    ///0x2d04 - RX Message Buffer Pointer Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr10_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(10)
    }
    ///0x2d84 - RX Message Buffer Pointer Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr11_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(11)
    }
    ///0x2e04 - RX Message Buffer Pointer Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr12_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(12)
    }
    ///0x2e84 - RX Message Buffer Pointer Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr13_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(13)
    }
    ///0x2f04 - RX Message Buffer Pointer Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr14_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(14)
    }
    ///0x2f84 - RX Message Buffer Pointer Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmptr15_1(&self) -> &Cfdrmptr1 {
        self.cfdrmptr_1(15)
    }
    ///0x2808..0x2848 - RX Message Buffer CAN-FD Status Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts_1(&self, n: usize) -> &Cfdrmfdsts1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10248).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2808..0x2848 - RX Message Buffer CAN-FD Status Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmfdsts_1_iter(&self) -> impl Iterator<Item = &Cfdrmfdsts1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10248).add(128 * n).cast()
            })
    }
    ///0x2808 - RX Message Buffer CAN-FD Status Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts0_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(0)
    }
    ///0x2888 - RX Message Buffer CAN-FD Status Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts1_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(1)
    }
    ///0x2908 - RX Message Buffer CAN-FD Status Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts2_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(2)
    }
    ///0x2988 - RX Message Buffer CAN-FD Status Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts3_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(3)
    }
    ///0x2a08 - RX Message Buffer CAN-FD Status Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts4_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(4)
    }
    ///0x2a88 - RX Message Buffer CAN-FD Status Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts5_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(5)
    }
    ///0x2b08 - RX Message Buffer CAN-FD Status Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts6_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(6)
    }
    ///0x2b88 - RX Message Buffer CAN-FD Status Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts7_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(7)
    }
    ///0x2c08 - RX Message Buffer CAN-FD Status Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts8_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(8)
    }
    ///0x2c88 - RX Message Buffer CAN-FD Status Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts9_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(9)
    }
    ///0x2d08 - RX Message Buffer CAN-FD Status Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts10_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(10)
    }
    ///0x2d88 - RX Message Buffer CAN-FD Status Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts11_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(11)
    }
    ///0x2e08 - RX Message Buffer CAN-FD Status Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts12_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(12)
    }
    ///0x2e88 - RX Message Buffer CAN-FD Status Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts13_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(13)
    }
    ///0x2f08 - RX Message Buffer CAN-FD Status Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts14_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(14)
    }
    ///0x2f88 - RX Message Buffer CAN-FD Status Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmfdsts15_1(&self) -> &Cfdrmfdsts1 {
        self.cfdrmfdsts_1(15)
    }
    ///0x280c..0x284c - RX Message Buffer Data Field 0 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0__1(&self, n: usize) -> &Cfdrmdf0_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10252).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x280c..0x284c - RX Message Buffer Data Field 0 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf0__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf0_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10252).add(128 * n).cast()
            })
    }
    ///0x280c - RX Message Buffer Data Field 0 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_0_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(0)
    }
    ///0x288c - RX Message Buffer Data Field 0 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_1_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(1)
    }
    ///0x290c - RX Message Buffer Data Field 0 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_2_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(2)
    }
    ///0x298c - RX Message Buffer Data Field 0 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_3_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(3)
    }
    ///0x2a0c - RX Message Buffer Data Field 0 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_4_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(4)
    }
    ///0x2a8c - RX Message Buffer Data Field 0 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_5_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(5)
    }
    ///0x2b0c - RX Message Buffer Data Field 0 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_6_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(6)
    }
    ///0x2b8c - RX Message Buffer Data Field 0 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_7_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(7)
    }
    ///0x2c0c - RX Message Buffer Data Field 0 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_8_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(8)
    }
    ///0x2c8c - RX Message Buffer Data Field 0 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_9_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(9)
    }
    ///0x2d0c - RX Message Buffer Data Field 0 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_10_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(10)
    }
    ///0x2d8c - RX Message Buffer Data Field 0 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_11_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(11)
    }
    ///0x2e0c - RX Message Buffer Data Field 0 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_12_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(12)
    }
    ///0x2e8c - RX Message Buffer Data Field 0 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_13_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(13)
    }
    ///0x2f0c - RX Message Buffer Data Field 0 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_14_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(14)
    }
    ///0x2f8c - RX Message Buffer Data Field 0 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf0_15_1(&self) -> &Cfdrmdf0_1 {
        self.cfdrmdf0__1(15)
    }
    ///0x2810..0x2850 - RX Message Buffer Data Field 1 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1__1(&self, n: usize) -> &Cfdrmdf1_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10256).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2810..0x2850 - RX Message Buffer Data Field 1 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf1__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf1_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10256).add(128 * n).cast()
            })
    }
    ///0x2810 - RX Message Buffer Data Field 1 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_0_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(0)
    }
    ///0x2890 - RX Message Buffer Data Field 1 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_1_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(1)
    }
    ///0x2910 - RX Message Buffer Data Field 1 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_2_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(2)
    }
    ///0x2990 - RX Message Buffer Data Field 1 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_3_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(3)
    }
    ///0x2a10 - RX Message Buffer Data Field 1 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_4_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(4)
    }
    ///0x2a90 - RX Message Buffer Data Field 1 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_5_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(5)
    }
    ///0x2b10 - RX Message Buffer Data Field 1 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_6_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(6)
    }
    ///0x2b90 - RX Message Buffer Data Field 1 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_7_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(7)
    }
    ///0x2c10 - RX Message Buffer Data Field 1 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_8_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(8)
    }
    ///0x2c90 - RX Message Buffer Data Field 1 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_9_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(9)
    }
    ///0x2d10 - RX Message Buffer Data Field 1 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_10_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(10)
    }
    ///0x2d90 - RX Message Buffer Data Field 1 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_11_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(11)
    }
    ///0x2e10 - RX Message Buffer Data Field 1 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_12_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(12)
    }
    ///0x2e90 - RX Message Buffer Data Field 1 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_13_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(13)
    }
    ///0x2f10 - RX Message Buffer Data Field 1 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_14_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(14)
    }
    ///0x2f90 - RX Message Buffer Data Field 1 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf1_15_1(&self) -> &Cfdrmdf1_1 {
        self.cfdrmdf1__1(15)
    }
    ///0x2814..0x2854 - RX Message Buffer Data Field 2 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2__1(&self, n: usize) -> &Cfdrmdf2_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10260).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2814..0x2854 - RX Message Buffer Data Field 2 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf2__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf2_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10260).add(128 * n).cast()
            })
    }
    ///0x2814 - RX Message Buffer Data Field 2 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_0_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(0)
    }
    ///0x2894 - RX Message Buffer Data Field 2 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_1_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(1)
    }
    ///0x2914 - RX Message Buffer Data Field 2 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_2_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(2)
    }
    ///0x2994 - RX Message Buffer Data Field 2 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_3_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(3)
    }
    ///0x2a14 - RX Message Buffer Data Field 2 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_4_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(4)
    }
    ///0x2a94 - RX Message Buffer Data Field 2 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_5_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(5)
    }
    ///0x2b14 - RX Message Buffer Data Field 2 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_6_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(6)
    }
    ///0x2b94 - RX Message Buffer Data Field 2 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_7_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(7)
    }
    ///0x2c14 - RX Message Buffer Data Field 2 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_8_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(8)
    }
    ///0x2c94 - RX Message Buffer Data Field 2 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_9_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(9)
    }
    ///0x2d14 - RX Message Buffer Data Field 2 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_10_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(10)
    }
    ///0x2d94 - RX Message Buffer Data Field 2 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_11_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(11)
    }
    ///0x2e14 - RX Message Buffer Data Field 2 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_12_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(12)
    }
    ///0x2e94 - RX Message Buffer Data Field 2 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_13_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(13)
    }
    ///0x2f14 - RX Message Buffer Data Field 2 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_14_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(14)
    }
    ///0x2f94 - RX Message Buffer Data Field 2 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf2_15_1(&self) -> &Cfdrmdf2_1 {
        self.cfdrmdf2__1(15)
    }
    ///0x2818..0x2858 - RX Message Buffer Data Field 3 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3__1(&self, n: usize) -> &Cfdrmdf3_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10264).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2818..0x2858 - RX Message Buffer Data Field 3 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf3__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf3_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10264).add(128 * n).cast()
            })
    }
    ///0x2818 - RX Message Buffer Data Field 3 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_0_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(0)
    }
    ///0x2898 - RX Message Buffer Data Field 3 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_1_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(1)
    }
    ///0x2918 - RX Message Buffer Data Field 3 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_2_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(2)
    }
    ///0x2998 - RX Message Buffer Data Field 3 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_3_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(3)
    }
    ///0x2a18 - RX Message Buffer Data Field 3 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_4_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(4)
    }
    ///0x2a98 - RX Message Buffer Data Field 3 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_5_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(5)
    }
    ///0x2b18 - RX Message Buffer Data Field 3 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_6_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(6)
    }
    ///0x2b98 - RX Message Buffer Data Field 3 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_7_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(7)
    }
    ///0x2c18 - RX Message Buffer Data Field 3 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_8_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(8)
    }
    ///0x2c98 - RX Message Buffer Data Field 3 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_9_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(9)
    }
    ///0x2d18 - RX Message Buffer Data Field 3 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_10_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(10)
    }
    ///0x2d98 - RX Message Buffer Data Field 3 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_11_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(11)
    }
    ///0x2e18 - RX Message Buffer Data Field 3 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_12_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(12)
    }
    ///0x2e98 - RX Message Buffer Data Field 3 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_13_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(13)
    }
    ///0x2f18 - RX Message Buffer Data Field 3 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_14_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(14)
    }
    ///0x2f98 - RX Message Buffer Data Field 3 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf3_15_1(&self) -> &Cfdrmdf3_1 {
        self.cfdrmdf3__1(15)
    }
    ///0x281c..0x285c - RX Message Buffer Data Field 4 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4__1(&self, n: usize) -> &Cfdrmdf4_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10268).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x281c..0x285c - RX Message Buffer Data Field 4 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf4__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf4_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10268).add(128 * n).cast()
            })
    }
    ///0x281c - RX Message Buffer Data Field 4 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_0_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(0)
    }
    ///0x289c - RX Message Buffer Data Field 4 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_1_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(1)
    }
    ///0x291c - RX Message Buffer Data Field 4 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_2_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(2)
    }
    ///0x299c - RX Message Buffer Data Field 4 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_3_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(3)
    }
    ///0x2a1c - RX Message Buffer Data Field 4 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_4_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(4)
    }
    ///0x2a9c - RX Message Buffer Data Field 4 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_5_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(5)
    }
    ///0x2b1c - RX Message Buffer Data Field 4 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_6_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(6)
    }
    ///0x2b9c - RX Message Buffer Data Field 4 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_7_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(7)
    }
    ///0x2c1c - RX Message Buffer Data Field 4 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_8_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(8)
    }
    ///0x2c9c - RX Message Buffer Data Field 4 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_9_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(9)
    }
    ///0x2d1c - RX Message Buffer Data Field 4 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_10_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(10)
    }
    ///0x2d9c - RX Message Buffer Data Field 4 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_11_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(11)
    }
    ///0x2e1c - RX Message Buffer Data Field 4 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_12_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(12)
    }
    ///0x2e9c - RX Message Buffer Data Field 4 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_13_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(13)
    }
    ///0x2f1c - RX Message Buffer Data Field 4 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_14_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(14)
    }
    ///0x2f9c - RX Message Buffer Data Field 4 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf4_15_1(&self) -> &Cfdrmdf4_1 {
        self.cfdrmdf4__1(15)
    }
    ///0x2820..0x2860 - RX Message Buffer Data Field 5 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5__1(&self, n: usize) -> &Cfdrmdf5_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10272).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2820..0x2860 - RX Message Buffer Data Field 5 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf5__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf5_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10272).add(128 * n).cast()
            })
    }
    ///0x2820 - RX Message Buffer Data Field 5 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_0_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(0)
    }
    ///0x28a0 - RX Message Buffer Data Field 5 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_1_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(1)
    }
    ///0x2920 - RX Message Buffer Data Field 5 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_2_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(2)
    }
    ///0x29a0 - RX Message Buffer Data Field 5 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_3_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(3)
    }
    ///0x2a20 - RX Message Buffer Data Field 5 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_4_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(4)
    }
    ///0x2aa0 - RX Message Buffer Data Field 5 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_5_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(5)
    }
    ///0x2b20 - RX Message Buffer Data Field 5 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_6_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(6)
    }
    ///0x2ba0 - RX Message Buffer Data Field 5 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_7_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(7)
    }
    ///0x2c20 - RX Message Buffer Data Field 5 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_8_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(8)
    }
    ///0x2ca0 - RX Message Buffer Data Field 5 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_9_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(9)
    }
    ///0x2d20 - RX Message Buffer Data Field 5 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_10_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(10)
    }
    ///0x2da0 - RX Message Buffer Data Field 5 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_11_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(11)
    }
    ///0x2e20 - RX Message Buffer Data Field 5 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_12_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(12)
    }
    ///0x2ea0 - RX Message Buffer Data Field 5 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_13_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(13)
    }
    ///0x2f20 - RX Message Buffer Data Field 5 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_14_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(14)
    }
    ///0x2fa0 - RX Message Buffer Data Field 5 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf5_15_1(&self) -> &Cfdrmdf5_1 {
        self.cfdrmdf5__1(15)
    }
    ///0x2824..0x2864 - RX Message Buffer Data Field 6 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6__1(&self, n: usize) -> &Cfdrmdf6_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10276).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2824..0x2864 - RX Message Buffer Data Field 6 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf6__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf6_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10276).add(128 * n).cast()
            })
    }
    ///0x2824 - RX Message Buffer Data Field 6 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_0_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(0)
    }
    ///0x28a4 - RX Message Buffer Data Field 6 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_1_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(1)
    }
    ///0x2924 - RX Message Buffer Data Field 6 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_2_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(2)
    }
    ///0x29a4 - RX Message Buffer Data Field 6 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_3_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(3)
    }
    ///0x2a24 - RX Message Buffer Data Field 6 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_4_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(4)
    }
    ///0x2aa4 - RX Message Buffer Data Field 6 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_5_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(5)
    }
    ///0x2b24 - RX Message Buffer Data Field 6 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_6_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(6)
    }
    ///0x2ba4 - RX Message Buffer Data Field 6 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_7_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(7)
    }
    ///0x2c24 - RX Message Buffer Data Field 6 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_8_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(8)
    }
    ///0x2ca4 - RX Message Buffer Data Field 6 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_9_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(9)
    }
    ///0x2d24 - RX Message Buffer Data Field 6 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_10_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(10)
    }
    ///0x2da4 - RX Message Buffer Data Field 6 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_11_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(11)
    }
    ///0x2e24 - RX Message Buffer Data Field 6 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_12_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(12)
    }
    ///0x2ea4 - RX Message Buffer Data Field 6 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_13_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(13)
    }
    ///0x2f24 - RX Message Buffer Data Field 6 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_14_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(14)
    }
    ///0x2fa4 - RX Message Buffer Data Field 6 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf6_15_1(&self) -> &Cfdrmdf6_1 {
        self.cfdrmdf6__1(15)
    }
    ///0x2828..0x2868 - RX Message Buffer Data Field 7 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7__1(&self, n: usize) -> &Cfdrmdf7_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10280).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2828..0x2868 - RX Message Buffer Data Field 7 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf7__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf7_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10280).add(128 * n).cast()
            })
    }
    ///0x2828 - RX Message Buffer Data Field 7 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_0_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(0)
    }
    ///0x28a8 - RX Message Buffer Data Field 7 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_1_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(1)
    }
    ///0x2928 - RX Message Buffer Data Field 7 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_2_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(2)
    }
    ///0x29a8 - RX Message Buffer Data Field 7 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_3_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(3)
    }
    ///0x2a28 - RX Message Buffer Data Field 7 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_4_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(4)
    }
    ///0x2aa8 - RX Message Buffer Data Field 7 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_5_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(5)
    }
    ///0x2b28 - RX Message Buffer Data Field 7 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_6_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(6)
    }
    ///0x2ba8 - RX Message Buffer Data Field 7 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_7_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(7)
    }
    ///0x2c28 - RX Message Buffer Data Field 7 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_8_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(8)
    }
    ///0x2ca8 - RX Message Buffer Data Field 7 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_9_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(9)
    }
    ///0x2d28 - RX Message Buffer Data Field 7 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_10_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(10)
    }
    ///0x2da8 - RX Message Buffer Data Field 7 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_11_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(11)
    }
    ///0x2e28 - RX Message Buffer Data Field 7 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_12_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(12)
    }
    ///0x2ea8 - RX Message Buffer Data Field 7 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_13_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(13)
    }
    ///0x2f28 - RX Message Buffer Data Field 7 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_14_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(14)
    }
    ///0x2fa8 - RX Message Buffer Data Field 7 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf7_15_1(&self) -> &Cfdrmdf7_1 {
        self.cfdrmdf7__1(15)
    }
    ///0x282c..0x286c - RX Message Buffer Data Field 8 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8__1(&self, n: usize) -> &Cfdrmdf8_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10284).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x282c..0x286c - RX Message Buffer Data Field 8 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf8__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf8_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10284).add(128 * n).cast()
            })
    }
    ///0x282c - RX Message Buffer Data Field 8 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_0_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(0)
    }
    ///0x28ac - RX Message Buffer Data Field 8 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_1_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(1)
    }
    ///0x292c - RX Message Buffer Data Field 8 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_2_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(2)
    }
    ///0x29ac - RX Message Buffer Data Field 8 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_3_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(3)
    }
    ///0x2a2c - RX Message Buffer Data Field 8 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_4_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(4)
    }
    ///0x2aac - RX Message Buffer Data Field 8 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_5_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(5)
    }
    ///0x2b2c - RX Message Buffer Data Field 8 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_6_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(6)
    }
    ///0x2bac - RX Message Buffer Data Field 8 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_7_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(7)
    }
    ///0x2c2c - RX Message Buffer Data Field 8 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_8_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(8)
    }
    ///0x2cac - RX Message Buffer Data Field 8 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_9_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(9)
    }
    ///0x2d2c - RX Message Buffer Data Field 8 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_10_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(10)
    }
    ///0x2dac - RX Message Buffer Data Field 8 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_11_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(11)
    }
    ///0x2e2c - RX Message Buffer Data Field 8 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_12_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(12)
    }
    ///0x2eac - RX Message Buffer Data Field 8 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_13_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(13)
    }
    ///0x2f2c - RX Message Buffer Data Field 8 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_14_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(14)
    }
    ///0x2fac - RX Message Buffer Data Field 8 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf8_15_1(&self) -> &Cfdrmdf8_1 {
        self.cfdrmdf8__1(15)
    }
    ///0x2830..0x2870 - RX Message Buffer Data Field 9 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9__1(&self, n: usize) -> &Cfdrmdf9_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10288).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2830..0x2870 - RX Message Buffer Data Field 9 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf9__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf9_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10288).add(128 * n).cast()
            })
    }
    ///0x2830 - RX Message Buffer Data Field 9 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_0_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(0)
    }
    ///0x28b0 - RX Message Buffer Data Field 9 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_1_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(1)
    }
    ///0x2930 - RX Message Buffer Data Field 9 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_2_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(2)
    }
    ///0x29b0 - RX Message Buffer Data Field 9 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_3_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(3)
    }
    ///0x2a30 - RX Message Buffer Data Field 9 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_4_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(4)
    }
    ///0x2ab0 - RX Message Buffer Data Field 9 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_5_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(5)
    }
    ///0x2b30 - RX Message Buffer Data Field 9 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_6_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(6)
    }
    ///0x2bb0 - RX Message Buffer Data Field 9 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_7_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(7)
    }
    ///0x2c30 - RX Message Buffer Data Field 9 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_8_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(8)
    }
    ///0x2cb0 - RX Message Buffer Data Field 9 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_9_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(9)
    }
    ///0x2d30 - RX Message Buffer Data Field 9 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_10_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(10)
    }
    ///0x2db0 - RX Message Buffer Data Field 9 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_11_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(11)
    }
    ///0x2e30 - RX Message Buffer Data Field 9 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_12_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(12)
    }
    ///0x2eb0 - RX Message Buffer Data Field 9 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_13_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(13)
    }
    ///0x2f30 - RX Message Buffer Data Field 9 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_14_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(14)
    }
    ///0x2fb0 - RX Message Buffer Data Field 9 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf9_15_1(&self) -> &Cfdrmdf9_1 {
        self.cfdrmdf9__1(15)
    }
    ///0x2834..0x2874 - RX Message Buffer Data Field 10 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10__1(&self, n: usize) -> &Cfdrmdf10_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10292).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2834..0x2874 - RX Message Buffer Data Field 10 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf10__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf10_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10292).add(128 * n).cast()
            })
    }
    ///0x2834 - RX Message Buffer Data Field 10 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_0_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(0)
    }
    ///0x28b4 - RX Message Buffer Data Field 10 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_1_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(1)
    }
    ///0x2934 - RX Message Buffer Data Field 10 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_2_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(2)
    }
    ///0x29b4 - RX Message Buffer Data Field 10 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_3_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(3)
    }
    ///0x2a34 - RX Message Buffer Data Field 10 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_4_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(4)
    }
    ///0x2ab4 - RX Message Buffer Data Field 10 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_5_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(5)
    }
    ///0x2b34 - RX Message Buffer Data Field 10 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_6_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(6)
    }
    ///0x2bb4 - RX Message Buffer Data Field 10 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_7_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(7)
    }
    ///0x2c34 - RX Message Buffer Data Field 10 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_8_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(8)
    }
    ///0x2cb4 - RX Message Buffer Data Field 10 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_9_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(9)
    }
    ///0x2d34 - RX Message Buffer Data Field 10 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_10_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(10)
    }
    ///0x2db4 - RX Message Buffer Data Field 10 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_11_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(11)
    }
    ///0x2e34 - RX Message Buffer Data Field 10 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_12_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(12)
    }
    ///0x2eb4 - RX Message Buffer Data Field 10 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_13_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(13)
    }
    ///0x2f34 - RX Message Buffer Data Field 10 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_14_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(14)
    }
    ///0x2fb4 - RX Message Buffer Data Field 10 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf10_15_1(&self) -> &Cfdrmdf10_1 {
        self.cfdrmdf10__1(15)
    }
    ///0x2838..0x2878 - RX Message Buffer Data Field 11 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11__1(&self, n: usize) -> &Cfdrmdf11_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10296).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2838..0x2878 - RX Message Buffer Data Field 11 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf11__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf11_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10296).add(128 * n).cast()
            })
    }
    ///0x2838 - RX Message Buffer Data Field 11 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_0_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(0)
    }
    ///0x28b8 - RX Message Buffer Data Field 11 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_1_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(1)
    }
    ///0x2938 - RX Message Buffer Data Field 11 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_2_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(2)
    }
    ///0x29b8 - RX Message Buffer Data Field 11 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_3_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(3)
    }
    ///0x2a38 - RX Message Buffer Data Field 11 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_4_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(4)
    }
    ///0x2ab8 - RX Message Buffer Data Field 11 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_5_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(5)
    }
    ///0x2b38 - RX Message Buffer Data Field 11 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_6_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(6)
    }
    ///0x2bb8 - RX Message Buffer Data Field 11 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_7_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(7)
    }
    ///0x2c38 - RX Message Buffer Data Field 11 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_8_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(8)
    }
    ///0x2cb8 - RX Message Buffer Data Field 11 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_9_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(9)
    }
    ///0x2d38 - RX Message Buffer Data Field 11 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_10_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(10)
    }
    ///0x2db8 - RX Message Buffer Data Field 11 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_11_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(11)
    }
    ///0x2e38 - RX Message Buffer Data Field 11 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_12_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(12)
    }
    ///0x2eb8 - RX Message Buffer Data Field 11 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_13_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(13)
    }
    ///0x2f38 - RX Message Buffer Data Field 11 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_14_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(14)
    }
    ///0x2fb8 - RX Message Buffer Data Field 11 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf11_15_1(&self) -> &Cfdrmdf11_1 {
        self.cfdrmdf11__1(15)
    }
    ///0x283c..0x287c - RX Message Buffer Data Field 12 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12__1(&self, n: usize) -> &Cfdrmdf12_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10300).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x283c..0x287c - RX Message Buffer Data Field 12 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf12__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf12_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10300).add(128 * n).cast()
            })
    }
    ///0x283c - RX Message Buffer Data Field 12 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_0_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(0)
    }
    ///0x28bc - RX Message Buffer Data Field 12 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_1_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(1)
    }
    ///0x293c - RX Message Buffer Data Field 12 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_2_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(2)
    }
    ///0x29bc - RX Message Buffer Data Field 12 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_3_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(3)
    }
    ///0x2a3c - RX Message Buffer Data Field 12 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_4_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(4)
    }
    ///0x2abc - RX Message Buffer Data Field 12 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_5_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(5)
    }
    ///0x2b3c - RX Message Buffer Data Field 12 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_6_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(6)
    }
    ///0x2bbc - RX Message Buffer Data Field 12 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_7_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(7)
    }
    ///0x2c3c - RX Message Buffer Data Field 12 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_8_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(8)
    }
    ///0x2cbc - RX Message Buffer Data Field 12 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_9_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(9)
    }
    ///0x2d3c - RX Message Buffer Data Field 12 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_10_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(10)
    }
    ///0x2dbc - RX Message Buffer Data Field 12 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_11_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(11)
    }
    ///0x2e3c - RX Message Buffer Data Field 12 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_12_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(12)
    }
    ///0x2ebc - RX Message Buffer Data Field 12 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_13_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(13)
    }
    ///0x2f3c - RX Message Buffer Data Field 12 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_14_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(14)
    }
    ///0x2fbc - RX Message Buffer Data Field 12 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf12_15_1(&self) -> &Cfdrmdf12_1 {
        self.cfdrmdf12__1(15)
    }
    ///0x2840..0x2880 - RX Message Buffer Data Field 13 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13__1(&self, n: usize) -> &Cfdrmdf13_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10304).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2840..0x2880 - RX Message Buffer Data Field 13 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf13__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf13_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10304).add(128 * n).cast()
            })
    }
    ///0x2840 - RX Message Buffer Data Field 13 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_0_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(0)
    }
    ///0x28c0 - RX Message Buffer Data Field 13 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_1_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(1)
    }
    ///0x2940 - RX Message Buffer Data Field 13 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_2_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(2)
    }
    ///0x29c0 - RX Message Buffer Data Field 13 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_3_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(3)
    }
    ///0x2a40 - RX Message Buffer Data Field 13 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_4_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(4)
    }
    ///0x2ac0 - RX Message Buffer Data Field 13 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_5_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(5)
    }
    ///0x2b40 - RX Message Buffer Data Field 13 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_6_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(6)
    }
    ///0x2bc0 - RX Message Buffer Data Field 13 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_7_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(7)
    }
    ///0x2c40 - RX Message Buffer Data Field 13 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_8_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(8)
    }
    ///0x2cc0 - RX Message Buffer Data Field 13 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_9_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(9)
    }
    ///0x2d40 - RX Message Buffer Data Field 13 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_10_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(10)
    }
    ///0x2dc0 - RX Message Buffer Data Field 13 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_11_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(11)
    }
    ///0x2e40 - RX Message Buffer Data Field 13 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_12_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(12)
    }
    ///0x2ec0 - RX Message Buffer Data Field 13 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_13_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(13)
    }
    ///0x2f40 - RX Message Buffer Data Field 13 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_14_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(14)
    }
    ///0x2fc0 - RX Message Buffer Data Field 13 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf13_15_1(&self) -> &Cfdrmdf13_1 {
        self.cfdrmdf13__1(15)
    }
    ///0x2844..0x2884 - RX Message Buffer Data Field 14 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14__1(&self, n: usize) -> &Cfdrmdf14_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10308).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2844..0x2884 - RX Message Buffer Data Field 14 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf14__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf14_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10308).add(128 * n).cast()
            })
    }
    ///0x2844 - RX Message Buffer Data Field 14 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_0_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(0)
    }
    ///0x28c4 - RX Message Buffer Data Field 14 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_1_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(1)
    }
    ///0x2944 - RX Message Buffer Data Field 14 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_2_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(2)
    }
    ///0x29c4 - RX Message Buffer Data Field 14 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_3_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(3)
    }
    ///0x2a44 - RX Message Buffer Data Field 14 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_4_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(4)
    }
    ///0x2ac4 - RX Message Buffer Data Field 14 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_5_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(5)
    }
    ///0x2b44 - RX Message Buffer Data Field 14 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_6_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(6)
    }
    ///0x2bc4 - RX Message Buffer Data Field 14 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_7_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(7)
    }
    ///0x2c44 - RX Message Buffer Data Field 14 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_8_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(8)
    }
    ///0x2cc4 - RX Message Buffer Data Field 14 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_9_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(9)
    }
    ///0x2d44 - RX Message Buffer Data Field 14 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_10_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(10)
    }
    ///0x2dc4 - RX Message Buffer Data Field 14 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_11_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(11)
    }
    ///0x2e44 - RX Message Buffer Data Field 14 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_12_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(12)
    }
    ///0x2ec4 - RX Message Buffer Data Field 14 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_13_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(13)
    }
    ///0x2f44 - RX Message Buffer Data Field 14 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_14_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(14)
    }
    ///0x2fc4 - RX Message Buffer Data Field 14 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf14_15_1(&self) -> &Cfdrmdf14_1 {
        self.cfdrmdf14__1(15)
    }
    ///0x2848..0x2888 - RX Message Buffer Data Field 15 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15__1(&self, n: usize) -> &Cfdrmdf15_1 {
        #[allow(clippy::no_effect)] [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(10312).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x2848..0x2888 - RX Message Buffer Data Field 15 Register %s Channel 1
    #[inline(always)]
    pub fn cfdrmdf15__1_iter(&self) -> impl Iterator<Item = &Cfdrmdf15_1> {
        (0..16)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(10312).add(128 * n).cast()
            })
    }
    ///0x2848 - RX Message Buffer Data Field 15 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_0_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(0)
    }
    ///0x28c8 - RX Message Buffer Data Field 15 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_1_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(1)
    }
    ///0x2948 - RX Message Buffer Data Field 15 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_2_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(2)
    }
    ///0x29c8 - RX Message Buffer Data Field 15 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_3_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(3)
    }
    ///0x2a48 - RX Message Buffer Data Field 15 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_4_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(4)
    }
    ///0x2ac8 - RX Message Buffer Data Field 15 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_5_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(5)
    }
    ///0x2b48 - RX Message Buffer Data Field 15 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_6_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(6)
    }
    ///0x2bc8 - RX Message Buffer Data Field 15 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_7_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(7)
    }
    ///0x2c48 - RX Message Buffer Data Field 15 Register 8 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_8_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(8)
    }
    ///0x2cc8 - RX Message Buffer Data Field 15 Register 9 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_9_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(9)
    }
    ///0x2d48 - RX Message Buffer Data Field 15 Register 10 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_10_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(10)
    }
    ///0x2dc8 - RX Message Buffer Data Field 15 Register 11 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_11_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(11)
    }
    ///0x2e48 - RX Message Buffer Data Field 15 Register 12 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_12_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(12)
    }
    ///0x2ec8 - RX Message Buffer Data Field 15 Register 13 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_13_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(13)
    }
    ///0x2f48 - RX Message Buffer Data Field 15 Register 14 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_14_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(14)
    }
    ///0x2fc8 - RX Message Buffer Data Field 15 Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdrmdf15_15_1(&self) -> &Cfdrmdf15_1 {
        self.cfdrmdf15__1(15)
    }
    ///0x6000..0x6020 - RX FIFO Access ID Register %s
    #[inline(always)]
    pub const fn cfdrfid(&self, n: usize) -> &Cfdrfid {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24576).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6000..0x6020 - RX FIFO Access ID Register %s
    #[inline(always)]
    pub fn cfdrfid_iter(&self) -> impl Iterator<Item = &Cfdrfid> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24576).add(128 * n).cast()
            })
    }
    ///0x6004..0x6024 - RX FIFO Access Pointer Register %s
    #[inline(always)]
    pub const fn cfdrfptr(&self, n: usize) -> &Cfdrfptr {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24580).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6004..0x6024 - RX FIFO Access Pointer Register %s
    #[inline(always)]
    pub fn cfdrfptr_iter(&self) -> impl Iterator<Item = &Cfdrfptr> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24580).add(128 * n).cast()
            })
    }
    ///0x6008..0x6028 - RX FIFO Access CAN-FD Status Register %s
    #[inline(always)]
    pub const fn cfdrffdsts(&self, n: usize) -> &Cfdrffdsts {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24584).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6008..0x6028 - RX FIFO Access CAN-FD Status Register %s
    #[inline(always)]
    pub fn cfdrffdsts_iter(&self) -> impl Iterator<Item = &Cfdrffdsts> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24584).add(128 * n).cast()
            })
    }
    ///0x600c..0x602c - RX FIFO Access Data Field 0 Register %s
    #[inline(always)]
    pub const fn cfdrfdf0(&self, n: usize) -> &Cfdrfdf0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24588).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x600c..0x602c - RX FIFO Access Data Field 0 Register %s
    #[inline(always)]
    pub fn cfdrfdf0_iter(&self) -> impl Iterator<Item = &Cfdrfdf0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24588).add(128 * n).cast()
            })
    }
    ///0x6010..0x6030 - RX FIFO Access Data Field 1 Register %s
    #[inline(always)]
    pub const fn cfdrfdf1(&self, n: usize) -> &Cfdrfdf1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24592).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6010..0x6030 - RX FIFO Access Data Field 1 Register %s
    #[inline(always)]
    pub fn cfdrfdf1_iter(&self) -> impl Iterator<Item = &Cfdrfdf1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24592).add(128 * n).cast()
            })
    }
    ///0x6014..0x6034 - RX FIFO Access Data Field 2 Register %s
    #[inline(always)]
    pub const fn cfdrfdf2(&self, n: usize) -> &Cfdrfdf2 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24596).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6014..0x6034 - RX FIFO Access Data Field 2 Register %s
    #[inline(always)]
    pub fn cfdrfdf2_iter(&self) -> impl Iterator<Item = &Cfdrfdf2> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24596).add(128 * n).cast()
            })
    }
    ///0x6018..0x6038 - RX FIFO Access Data Field 3 Register %s
    #[inline(always)]
    pub const fn cfdrfdf3(&self, n: usize) -> &Cfdrfdf3 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24600).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6018..0x6038 - RX FIFO Access Data Field 3 Register %s
    #[inline(always)]
    pub fn cfdrfdf3_iter(&self) -> impl Iterator<Item = &Cfdrfdf3> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24600).add(128 * n).cast()
            })
    }
    ///0x601c..0x603c - RX FIFO Access Data Field 4 Register %s
    #[inline(always)]
    pub const fn cfdrfdf4(&self, n: usize) -> &Cfdrfdf4 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24604).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x601c..0x603c - RX FIFO Access Data Field 4 Register %s
    #[inline(always)]
    pub fn cfdrfdf4_iter(&self) -> impl Iterator<Item = &Cfdrfdf4> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24604).add(128 * n).cast()
            })
    }
    ///0x6020..0x6040 - RX FIFO Access Data Field 5 Register %s
    #[inline(always)]
    pub const fn cfdrfdf5(&self, n: usize) -> &Cfdrfdf5 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24608).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6020..0x6040 - RX FIFO Access Data Field 5 Register %s
    #[inline(always)]
    pub fn cfdrfdf5_iter(&self) -> impl Iterator<Item = &Cfdrfdf5> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24608).add(128 * n).cast()
            })
    }
    ///0x6024..0x6044 - RX FIFO Access Data Field 6 Register %s
    #[inline(always)]
    pub const fn cfdrfdf6(&self, n: usize) -> &Cfdrfdf6 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24612).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6024..0x6044 - RX FIFO Access Data Field 6 Register %s
    #[inline(always)]
    pub fn cfdrfdf6_iter(&self) -> impl Iterator<Item = &Cfdrfdf6> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24612).add(128 * n).cast()
            })
    }
    ///0x6028..0x6048 - RX FIFO Access Data Field 7 Register %s
    #[inline(always)]
    pub const fn cfdrfdf7(&self, n: usize) -> &Cfdrfdf7 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24616).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6028..0x6048 - RX FIFO Access Data Field 7 Register %s
    #[inline(always)]
    pub fn cfdrfdf7_iter(&self) -> impl Iterator<Item = &Cfdrfdf7> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24616).add(128 * n).cast()
            })
    }
    ///0x602c..0x604c - RX FIFO Access Data Field 8 Register %s
    #[inline(always)]
    pub const fn cfdrfdf8(&self, n: usize) -> &Cfdrfdf8 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24620).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x602c..0x604c - RX FIFO Access Data Field 8 Register %s
    #[inline(always)]
    pub fn cfdrfdf8_iter(&self) -> impl Iterator<Item = &Cfdrfdf8> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24620).add(128 * n).cast()
            })
    }
    ///0x6030..0x6050 - RX FIFO Access Data Field 9 Register %s
    #[inline(always)]
    pub const fn cfdrfdf9(&self, n: usize) -> &Cfdrfdf9 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24624).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6030..0x6050 - RX FIFO Access Data Field 9 Register %s
    #[inline(always)]
    pub fn cfdrfdf9_iter(&self) -> impl Iterator<Item = &Cfdrfdf9> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24624).add(128 * n).cast()
            })
    }
    ///0x6034..0x6054 - RX FIFO Access Data Field 10 Register %s
    #[inline(always)]
    pub const fn cfdrfdf10(&self, n: usize) -> &Cfdrfdf10 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24628).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6034..0x6054 - RX FIFO Access Data Field 10 Register %s
    #[inline(always)]
    pub fn cfdrfdf10_iter(&self) -> impl Iterator<Item = &Cfdrfdf10> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24628).add(128 * n).cast()
            })
    }
    ///0x6038..0x6058 - RX FIFO Access Data Field 11 Register %s
    #[inline(always)]
    pub const fn cfdrfdf11(&self, n: usize) -> &Cfdrfdf11 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24632).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6038..0x6058 - RX FIFO Access Data Field 11 Register %s
    #[inline(always)]
    pub fn cfdrfdf11_iter(&self) -> impl Iterator<Item = &Cfdrfdf11> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24632).add(128 * n).cast()
            })
    }
    ///0x603c..0x605c - RX FIFO Access Data Field 12 Register %s
    #[inline(always)]
    pub const fn cfdrfdf12(&self, n: usize) -> &Cfdrfdf12 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24636).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x603c..0x605c - RX FIFO Access Data Field 12 Register %s
    #[inline(always)]
    pub fn cfdrfdf12_iter(&self) -> impl Iterator<Item = &Cfdrfdf12> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24636).add(128 * n).cast()
            })
    }
    ///0x6040..0x6060 - RX FIFO Access Data Field 13 Register %s
    #[inline(always)]
    pub const fn cfdrfdf13(&self, n: usize) -> &Cfdrfdf13 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24640).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6040..0x6060 - RX FIFO Access Data Field 13 Register %s
    #[inline(always)]
    pub fn cfdrfdf13_iter(&self) -> impl Iterator<Item = &Cfdrfdf13> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24640).add(128 * n).cast()
            })
    }
    ///0x6044..0x6064 - RX FIFO Access Data Field 14 Register %s
    #[inline(always)]
    pub const fn cfdrfdf14(&self, n: usize) -> &Cfdrfdf14 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24644).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6044..0x6064 - RX FIFO Access Data Field 14 Register %s
    #[inline(always)]
    pub fn cfdrfdf14_iter(&self) -> impl Iterator<Item = &Cfdrfdf14> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24644).add(128 * n).cast()
            })
    }
    ///0x6048..0x6068 - RX FIFO Access Data Field 15 Register %s
    #[inline(always)]
    pub const fn cfdrfdf15(&self, n: usize) -> &Cfdrfdf15 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(24648).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6048..0x6068 - RX FIFO Access Data Field 15 Register %s
    #[inline(always)]
    pub fn cfdrfdf15_iter(&self) -> impl Iterator<Item = &Cfdrfdf15> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24648).add(128 * n).cast()
            })
    }
    ///0x6400..0x640c - Common FIFO Access ID Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfid_0(&self, n: usize) -> &Cfdcfid0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25600).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6400..0x640c - Common FIFO Access ID Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfid_0_iter(&self) -> impl Iterator<Item = &Cfdcfid0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25600).add(128 * n).cast()
            })
    }
    ///0x6400 - Common FIFO Access ID Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfid0_0(&self) -> &Cfdcfid0 {
        self.cfdcfid_0(0)
    }
    ///0x6480 - Common FIFO Access ID Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfid1_0(&self) -> &Cfdcfid0 {
        self.cfdcfid_0(1)
    }
    ///0x6500 - Common FIFO Access ID Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfid2_0(&self) -> &Cfdcfid0 {
        self.cfdcfid_0(2)
    }
    ///0x6404..0x6410 - Common FIFO Access Pointer Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfptr_0(&self, n: usize) -> &Cfdcfptr0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25604).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6404..0x6410 - Common FIFO Access Pointer Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfptr_0_iter(&self) -> impl Iterator<Item = &Cfdcfptr0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25604).add(128 * n).cast()
            })
    }
    ///0x6404 - Common FIFO Access Pointer Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfptr0_0(&self) -> &Cfdcfptr0 {
        self.cfdcfptr_0(0)
    }
    ///0x6484 - Common FIFO Access Pointer Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfptr1_0(&self) -> &Cfdcfptr0 {
        self.cfdcfptr_0(1)
    }
    ///0x6504 - Common FIFO Access Pointer Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfptr2_0(&self) -> &Cfdcfptr0 {
        self.cfdcfptr_0(2)
    }
    ///0x6408..0x6414 - Common FIFO Access CAN-FD Control/Status Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcffdcsts_0(&self, n: usize) -> &Cfdcffdcsts0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25608).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6408..0x6414 - Common FIFO Access CAN-FD Control/Status Register %s Channel 0
    #[inline(always)]
    pub fn cfdcffdcsts_0_iter(&self) -> impl Iterator<Item = &Cfdcffdcsts0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25608).add(128 * n).cast()
            })
    }
    ///0x6408 - Common FIFO Access CAN-FD Control/Status Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcffdcsts0_0(&self) -> &Cfdcffdcsts0 {
        self.cfdcffdcsts_0(0)
    }
    ///0x6488 - Common FIFO Access CAN-FD Control/Status Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcffdcsts1_0(&self) -> &Cfdcffdcsts0 {
        self.cfdcffdcsts_0(1)
    }
    ///0x6508 - Common FIFO Access CAN-FD Control/Status Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcffdcsts2_0(&self) -> &Cfdcffdcsts0 {
        self.cfdcffdcsts_0(2)
    }
    ///0x640c..0x6418 - Common FIFO Access Data Field 0 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf0_0(&self, n: usize) -> &Cfdcfdf0_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25612).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x640c..0x6418 - Common FIFO Access Data Field 0 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf0_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf0_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25612).add(128 * n).cast()
            })
    }
    ///0x640c - Common FIFO Access Data Field 0 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf00_0(&self) -> &Cfdcfdf0_0 {
        self.cfdcfdf0_0(0)
    }
    ///0x648c - Common FIFO Access Data Field 0 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf01_0(&self) -> &Cfdcfdf0_0 {
        self.cfdcfdf0_0(1)
    }
    ///0x650c - Common FIFO Access Data Field 0 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf02_0(&self) -> &Cfdcfdf0_0 {
        self.cfdcfdf0_0(2)
    }
    ///0x6410..0x641c - Common FIFO Access Data Field 1 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf1_0(&self, n: usize) -> &Cfdcfdf1_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25616).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6410..0x641c - Common FIFO Access Data Field 1 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf1_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf1_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25616).add(128 * n).cast()
            })
    }
    ///0x6410 - Common FIFO Access Data Field 1 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf10_0(&self) -> &Cfdcfdf1_0 {
        self.cfdcfdf1_0(0)
    }
    ///0x6490 - Common FIFO Access Data Field 1 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf11_0(&self) -> &Cfdcfdf1_0 {
        self.cfdcfdf1_0(1)
    }
    ///0x6510 - Common FIFO Access Data Field 1 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf12_0(&self) -> &Cfdcfdf1_0 {
        self.cfdcfdf1_0(2)
    }
    ///0x6414..0x6420 - Common FIFO Access Data Field 2 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf2_0(&self, n: usize) -> &Cfdcfdf2_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25620).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6414..0x6420 - Common FIFO Access Data Field 2 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf2_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf2_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25620).add(128 * n).cast()
            })
    }
    ///0x6414 - Common FIFO Access Data Field 2 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf20_0(&self) -> &Cfdcfdf2_0 {
        self.cfdcfdf2_0(0)
    }
    ///0x6494 - Common FIFO Access Data Field 2 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf21_0(&self) -> &Cfdcfdf2_0 {
        self.cfdcfdf2_0(1)
    }
    ///0x6514 - Common FIFO Access Data Field 2 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf22_0(&self) -> &Cfdcfdf2_0 {
        self.cfdcfdf2_0(2)
    }
    ///0x6418..0x6424 - Common FIFO Access Data Field 3 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf3_0(&self, n: usize) -> &Cfdcfdf3_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25624).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6418..0x6424 - Common FIFO Access Data Field 3 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf3_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf3_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25624).add(128 * n).cast()
            })
    }
    ///0x6418 - Common FIFO Access Data Field 3 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf30_0(&self) -> &Cfdcfdf3_0 {
        self.cfdcfdf3_0(0)
    }
    ///0x6498 - Common FIFO Access Data Field 3 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf31_0(&self) -> &Cfdcfdf3_0 {
        self.cfdcfdf3_0(1)
    }
    ///0x6518 - Common FIFO Access Data Field 3 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf32_0(&self) -> &Cfdcfdf3_0 {
        self.cfdcfdf3_0(2)
    }
    ///0x641c..0x6428 - Common FIFO Access Data Field 4 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf4_0(&self, n: usize) -> &Cfdcfdf4_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25628).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x641c..0x6428 - Common FIFO Access Data Field 4 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf4_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf4_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25628).add(128 * n).cast()
            })
    }
    ///0x641c - Common FIFO Access Data Field 4 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf40_0(&self) -> &Cfdcfdf4_0 {
        self.cfdcfdf4_0(0)
    }
    ///0x649c - Common FIFO Access Data Field 4 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf41_0(&self) -> &Cfdcfdf4_0 {
        self.cfdcfdf4_0(1)
    }
    ///0x651c - Common FIFO Access Data Field 4 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf42_0(&self) -> &Cfdcfdf4_0 {
        self.cfdcfdf4_0(2)
    }
    ///0x6420..0x642c - Common FIFO Access Data Field 5 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf5_0(&self, n: usize) -> &Cfdcfdf5_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25632).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6420..0x642c - Common FIFO Access Data Field 5 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf5_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf5_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25632).add(128 * n).cast()
            })
    }
    ///0x6420 - Common FIFO Access Data Field 5 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf50_0(&self) -> &Cfdcfdf5_0 {
        self.cfdcfdf5_0(0)
    }
    ///0x64a0 - Common FIFO Access Data Field 5 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf51_0(&self) -> &Cfdcfdf5_0 {
        self.cfdcfdf5_0(1)
    }
    ///0x6520 - Common FIFO Access Data Field 5 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf52_0(&self) -> &Cfdcfdf5_0 {
        self.cfdcfdf5_0(2)
    }
    ///0x6424..0x6430 - Common FIFO Access Data Field 6 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf6_0(&self, n: usize) -> &Cfdcfdf6_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25636).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6424..0x6430 - Common FIFO Access Data Field 6 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf6_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf6_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25636).add(128 * n).cast()
            })
    }
    ///0x6424 - Common FIFO Access Data Field 6 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf60_0(&self) -> &Cfdcfdf6_0 {
        self.cfdcfdf6_0(0)
    }
    ///0x64a4 - Common FIFO Access Data Field 6 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf61_0(&self) -> &Cfdcfdf6_0 {
        self.cfdcfdf6_0(1)
    }
    ///0x6524 - Common FIFO Access Data Field 6 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf62_0(&self) -> &Cfdcfdf6_0 {
        self.cfdcfdf6_0(2)
    }
    ///0x6428..0x6434 - Common FIFO Access Data Field 7 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf7_0(&self, n: usize) -> &Cfdcfdf7_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25640).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6428..0x6434 - Common FIFO Access Data Field 7 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf7_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf7_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25640).add(128 * n).cast()
            })
    }
    ///0x6428 - Common FIFO Access Data Field 7 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf70_0(&self) -> &Cfdcfdf7_0 {
        self.cfdcfdf7_0(0)
    }
    ///0x64a8 - Common FIFO Access Data Field 7 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf71_0(&self) -> &Cfdcfdf7_0 {
        self.cfdcfdf7_0(1)
    }
    ///0x6528 - Common FIFO Access Data Field 7 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf72_0(&self) -> &Cfdcfdf7_0 {
        self.cfdcfdf7_0(2)
    }
    ///0x642c..0x6438 - Common FIFO Access Data Field 8 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf8_0(&self, n: usize) -> &Cfdcfdf8_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25644).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x642c..0x6438 - Common FIFO Access Data Field 8 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf8_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf8_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25644).add(128 * n).cast()
            })
    }
    ///0x642c - Common FIFO Access Data Field 8 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf80_0(&self) -> &Cfdcfdf8_0 {
        self.cfdcfdf8_0(0)
    }
    ///0x64ac - Common FIFO Access Data Field 8 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf81_0(&self) -> &Cfdcfdf8_0 {
        self.cfdcfdf8_0(1)
    }
    ///0x652c - Common FIFO Access Data Field 8 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf82_0(&self) -> &Cfdcfdf8_0 {
        self.cfdcfdf8_0(2)
    }
    ///0x6430..0x643c - Common FIFO Access Data Field 9 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf9_0(&self, n: usize) -> &Cfdcfdf9_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25648).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6430..0x643c - Common FIFO Access Data Field 9 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf9_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf9_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25648).add(128 * n).cast()
            })
    }
    ///0x6430 - Common FIFO Access Data Field 9 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf90_0(&self) -> &Cfdcfdf9_0 {
        self.cfdcfdf9_0(0)
    }
    ///0x64b0 - Common FIFO Access Data Field 9 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf91_0(&self) -> &Cfdcfdf9_0 {
        self.cfdcfdf9_0(1)
    }
    ///0x6530 - Common FIFO Access Data Field 9 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf92_0(&self) -> &Cfdcfdf9_0 {
        self.cfdcfdf9_0(2)
    }
    ///0x6434..0x6440 - Common FIFO Access Data Field 10 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf10_0(&self, n: usize) -> &Cfdcfdf10_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25652).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6434..0x6440 - Common FIFO Access Data Field 10 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf10_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf10_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25652).add(128 * n).cast()
            })
    }
    ///0x6434 - Common FIFO Access Data Field 10 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf100_0(&self) -> &Cfdcfdf10_0 {
        self.cfdcfdf10_0(0)
    }
    ///0x64b4 - Common FIFO Access Data Field 10 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf101_0(&self) -> &Cfdcfdf10_0 {
        self.cfdcfdf10_0(1)
    }
    ///0x6534 - Common FIFO Access Data Field 10 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf102_0(&self) -> &Cfdcfdf10_0 {
        self.cfdcfdf10_0(2)
    }
    ///0x6438..0x6444 - Common FIFO Access Data Field 11 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf11_0(&self, n: usize) -> &Cfdcfdf11_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25656).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6438..0x6444 - Common FIFO Access Data Field 11 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf11_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf11_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25656).add(128 * n).cast()
            })
    }
    ///0x6438 - Common FIFO Access Data Field 11 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf110_0(&self) -> &Cfdcfdf11_0 {
        self.cfdcfdf11_0(0)
    }
    ///0x64b8 - Common FIFO Access Data Field 11 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf111_0(&self) -> &Cfdcfdf11_0 {
        self.cfdcfdf11_0(1)
    }
    ///0x6538 - Common FIFO Access Data Field 11 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf112_0(&self) -> &Cfdcfdf11_0 {
        self.cfdcfdf11_0(2)
    }
    ///0x643c..0x6448 - Common FIFO Access Data Field 12 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf12_0(&self, n: usize) -> &Cfdcfdf12_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25660).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x643c..0x6448 - Common FIFO Access Data Field 12 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf12_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf12_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25660).add(128 * n).cast()
            })
    }
    ///0x643c - Common FIFO Access Data Field 12 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf120_0(&self) -> &Cfdcfdf12_0 {
        self.cfdcfdf12_0(0)
    }
    ///0x64bc - Common FIFO Access Data Field 12 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf121_0(&self) -> &Cfdcfdf12_0 {
        self.cfdcfdf12_0(1)
    }
    ///0x653c - Common FIFO Access Data Field 12 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf122_0(&self) -> &Cfdcfdf12_0 {
        self.cfdcfdf12_0(2)
    }
    ///0x6440..0x644c - Common FIFO Access Data Field 13 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf13_0(&self, n: usize) -> &Cfdcfdf13_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25664).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6440..0x644c - Common FIFO Access Data Field 13 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf13_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf13_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25664).add(128 * n).cast()
            })
    }
    ///0x6440 - Common FIFO Access Data Field 13 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf130_0(&self) -> &Cfdcfdf13_0 {
        self.cfdcfdf13_0(0)
    }
    ///0x64c0 - Common FIFO Access Data Field 13 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf131_0(&self) -> &Cfdcfdf13_0 {
        self.cfdcfdf13_0(1)
    }
    ///0x6540 - Common FIFO Access Data Field 13 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf132_0(&self) -> &Cfdcfdf13_0 {
        self.cfdcfdf13_0(2)
    }
    ///0x6444..0x6450 - Common FIFO Access Data Field 14 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf14_0(&self, n: usize) -> &Cfdcfdf14_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25668).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6444..0x6450 - Common FIFO Access Data Field 14 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf14_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf14_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25668).add(128 * n).cast()
            })
    }
    ///0x6444 - Common FIFO Access Data Field 14 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf140_0(&self) -> &Cfdcfdf14_0 {
        self.cfdcfdf14_0(0)
    }
    ///0x64c4 - Common FIFO Access Data Field 14 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf141_0(&self) -> &Cfdcfdf14_0 {
        self.cfdcfdf14_0(1)
    }
    ///0x6544 - Common FIFO Access Data Field 14 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf142_0(&self) -> &Cfdcfdf14_0 {
        self.cfdcfdf14_0(2)
    }
    ///0x6448..0x6454 - Common FIFO Access Data Field 15 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdcfdf15_0(&self, n: usize) -> &Cfdcfdf15_0 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25672).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6448..0x6454 - Common FIFO Access Data Field 15 Register %s Channel 0
    #[inline(always)]
    pub fn cfdcfdf15_0_iter(&self) -> impl Iterator<Item = &Cfdcfdf15_0> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25672).add(128 * n).cast()
            })
    }
    ///0x6448 - Common FIFO Access Data Field 15 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf150_0(&self) -> &Cfdcfdf15_0 {
        self.cfdcfdf15_0(0)
    }
    ///0x64c8 - Common FIFO Access Data Field 15 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf151_0(&self) -> &Cfdcfdf15_0 {
        self.cfdcfdf15_0(1)
    }
    ///0x6548 - Common FIFO Access Data Field 15 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdcfdf152_0(&self) -> &Cfdcfdf15_0 {
        self.cfdcfdf15_0(2)
    }
    ///0x6580..0x658c - Common FIFO Access ID Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfid_1(&self, n: usize) -> &Cfdcfid1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25984).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6580..0x658c - Common FIFO Access ID Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfid_1_iter(&self) -> impl Iterator<Item = &Cfdcfid1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25984).add(128 * n).cast()
            })
    }
    ///0x6580 - Common FIFO Access ID Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfid0_1(&self) -> &Cfdcfid1 {
        self.cfdcfid_1(0)
    }
    ///0x6600 - Common FIFO Access ID Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfid1_1(&self) -> &Cfdcfid1 {
        self.cfdcfid_1(1)
    }
    ///0x6680 - Common FIFO Access ID Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfid2_1(&self) -> &Cfdcfid1 {
        self.cfdcfid_1(2)
    }
    ///0x6584..0x6590 - Common FIFO Access Pointer Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfptr_1(&self, n: usize) -> &Cfdcfptr1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25988).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6584..0x6590 - Common FIFO Access Pointer Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfptr_1_iter(&self) -> impl Iterator<Item = &Cfdcfptr1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25988).add(128 * n).cast()
            })
    }
    ///0x6584 - Common FIFO Access Pointer Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfptr0_1(&self) -> &Cfdcfptr1 {
        self.cfdcfptr_1(0)
    }
    ///0x6604 - Common FIFO Access Pointer Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfptr1_1(&self) -> &Cfdcfptr1 {
        self.cfdcfptr_1(1)
    }
    ///0x6684 - Common FIFO Access Pointer Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfptr2_1(&self) -> &Cfdcfptr1 {
        self.cfdcfptr_1(2)
    }
    ///0x6588..0x6594 - Common FIFO Access CAN-FD Control/Status Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcffdcsts_1(&self, n: usize) -> &Cfdcffdcsts1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25992).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6588..0x6594 - Common FIFO Access CAN-FD Control/Status Register %s Channel 1
    #[inline(always)]
    pub fn cfdcffdcsts_1_iter(&self) -> impl Iterator<Item = &Cfdcffdcsts1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25992).add(128 * n).cast()
            })
    }
    ///0x6588 - Common FIFO Access CAN-FD Control/Status Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcffdcsts0_1(&self) -> &Cfdcffdcsts1 {
        self.cfdcffdcsts_1(0)
    }
    ///0x6608 - Common FIFO Access CAN-FD Control/Status Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcffdcsts1_1(&self) -> &Cfdcffdcsts1 {
        self.cfdcffdcsts_1(1)
    }
    ///0x6688 - Common FIFO Access CAN-FD Control/Status Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcffdcsts2_1(&self) -> &Cfdcffdcsts1 {
        self.cfdcffdcsts_1(2)
    }
    ///0x658c..0x6598 - Common FIFO Access Data Field 0 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf0_1(&self, n: usize) -> &Cfdcfdf0_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(25996).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x658c..0x6598 - Common FIFO Access Data Field 0 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf0_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf0_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(25996).add(128 * n).cast()
            })
    }
    ///0x658c - Common FIFO Access Data Field 0 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf00_1(&self) -> &Cfdcfdf0_1 {
        self.cfdcfdf0_1(0)
    }
    ///0x660c - Common FIFO Access Data Field 0 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf01_1(&self) -> &Cfdcfdf0_1 {
        self.cfdcfdf0_1(1)
    }
    ///0x668c - Common FIFO Access Data Field 0 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf02_1(&self) -> &Cfdcfdf0_1 {
        self.cfdcfdf0_1(2)
    }
    ///0x6590..0x659c - Common FIFO Access Data Field 1 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf1_1(&self, n: usize) -> &Cfdcfdf1_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26000).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6590..0x659c - Common FIFO Access Data Field 1 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf1_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf1_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26000).add(128 * n).cast()
            })
    }
    ///0x6590 - Common FIFO Access Data Field 1 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf10_1(&self) -> &Cfdcfdf1_1 {
        self.cfdcfdf1_1(0)
    }
    ///0x6610 - Common FIFO Access Data Field 1 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf11_1(&self) -> &Cfdcfdf1_1 {
        self.cfdcfdf1_1(1)
    }
    ///0x6690 - Common FIFO Access Data Field 1 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf12_1(&self) -> &Cfdcfdf1_1 {
        self.cfdcfdf1_1(2)
    }
    ///0x6594..0x65a0 - Common FIFO Access Data Field 2 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf2_1(&self, n: usize) -> &Cfdcfdf2_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26004).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6594..0x65a0 - Common FIFO Access Data Field 2 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf2_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf2_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26004).add(128 * n).cast()
            })
    }
    ///0x6594 - Common FIFO Access Data Field 2 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf20_1(&self) -> &Cfdcfdf2_1 {
        self.cfdcfdf2_1(0)
    }
    ///0x6614 - Common FIFO Access Data Field 2 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf21_1(&self) -> &Cfdcfdf2_1 {
        self.cfdcfdf2_1(1)
    }
    ///0x6694 - Common FIFO Access Data Field 2 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf22_1(&self) -> &Cfdcfdf2_1 {
        self.cfdcfdf2_1(2)
    }
    ///0x6598..0x65a4 - Common FIFO Access Data Field 3 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf3_1(&self, n: usize) -> &Cfdcfdf3_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26008).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x6598..0x65a4 - Common FIFO Access Data Field 3 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf3_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf3_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26008).add(128 * n).cast()
            })
    }
    ///0x6598 - Common FIFO Access Data Field 3 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf30_1(&self) -> &Cfdcfdf3_1 {
        self.cfdcfdf3_1(0)
    }
    ///0x6618 - Common FIFO Access Data Field 3 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf31_1(&self) -> &Cfdcfdf3_1 {
        self.cfdcfdf3_1(1)
    }
    ///0x6698 - Common FIFO Access Data Field 3 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf32_1(&self) -> &Cfdcfdf3_1 {
        self.cfdcfdf3_1(2)
    }
    ///0x659c..0x65a8 - Common FIFO Access Data Field 4 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf4_1(&self, n: usize) -> &Cfdcfdf4_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26012).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x659c..0x65a8 - Common FIFO Access Data Field 4 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf4_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf4_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26012).add(128 * n).cast()
            })
    }
    ///0x659c - Common FIFO Access Data Field 4 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf40_1(&self) -> &Cfdcfdf4_1 {
        self.cfdcfdf4_1(0)
    }
    ///0x661c - Common FIFO Access Data Field 4 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf41_1(&self) -> &Cfdcfdf4_1 {
        self.cfdcfdf4_1(1)
    }
    ///0x669c - Common FIFO Access Data Field 4 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf42_1(&self) -> &Cfdcfdf4_1 {
        self.cfdcfdf4_1(2)
    }
    ///0x65a0..0x65ac - Common FIFO Access Data Field 5 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf5_1(&self, n: usize) -> &Cfdcfdf5_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26016).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65a0..0x65ac - Common FIFO Access Data Field 5 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf5_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf5_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26016).add(128 * n).cast()
            })
    }
    ///0x65a0 - Common FIFO Access Data Field 5 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf50_1(&self) -> &Cfdcfdf5_1 {
        self.cfdcfdf5_1(0)
    }
    ///0x6620 - Common FIFO Access Data Field 5 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf51_1(&self) -> &Cfdcfdf5_1 {
        self.cfdcfdf5_1(1)
    }
    ///0x66a0 - Common FIFO Access Data Field 5 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf52_1(&self) -> &Cfdcfdf5_1 {
        self.cfdcfdf5_1(2)
    }
    ///0x65a4..0x65b0 - Common FIFO Access Data Field 6 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf6_1(&self, n: usize) -> &Cfdcfdf6_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26020).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65a4..0x65b0 - Common FIFO Access Data Field 6 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf6_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf6_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26020).add(128 * n).cast()
            })
    }
    ///0x65a4 - Common FIFO Access Data Field 6 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf60_1(&self) -> &Cfdcfdf6_1 {
        self.cfdcfdf6_1(0)
    }
    ///0x6624 - Common FIFO Access Data Field 6 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf61_1(&self) -> &Cfdcfdf6_1 {
        self.cfdcfdf6_1(1)
    }
    ///0x66a4 - Common FIFO Access Data Field 6 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf62_1(&self) -> &Cfdcfdf6_1 {
        self.cfdcfdf6_1(2)
    }
    ///0x65a8..0x65b4 - Common FIFO Access Data Field 7 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf7_1(&self, n: usize) -> &Cfdcfdf7_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26024).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65a8..0x65b4 - Common FIFO Access Data Field 7 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf7_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf7_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26024).add(128 * n).cast()
            })
    }
    ///0x65a8 - Common FIFO Access Data Field 7 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf70_1(&self) -> &Cfdcfdf7_1 {
        self.cfdcfdf7_1(0)
    }
    ///0x6628 - Common FIFO Access Data Field 7 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf71_1(&self) -> &Cfdcfdf7_1 {
        self.cfdcfdf7_1(1)
    }
    ///0x66a8 - Common FIFO Access Data Field 7 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf72_1(&self) -> &Cfdcfdf7_1 {
        self.cfdcfdf7_1(2)
    }
    ///0x65ac..0x65b8 - Common FIFO Access Data Field 8 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf8_1(&self, n: usize) -> &Cfdcfdf8_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26028).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65ac..0x65b8 - Common FIFO Access Data Field 8 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf8_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf8_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26028).add(128 * n).cast()
            })
    }
    ///0x65ac - Common FIFO Access Data Field 8 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf80_1(&self) -> &Cfdcfdf8_1 {
        self.cfdcfdf8_1(0)
    }
    ///0x662c - Common FIFO Access Data Field 8 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf81_1(&self) -> &Cfdcfdf8_1 {
        self.cfdcfdf8_1(1)
    }
    ///0x66ac - Common FIFO Access Data Field 8 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf82_1(&self) -> &Cfdcfdf8_1 {
        self.cfdcfdf8_1(2)
    }
    ///0x65b0..0x65bc - Common FIFO Access Data Field 9 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf9_1(&self, n: usize) -> &Cfdcfdf9_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26032).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65b0..0x65bc - Common FIFO Access Data Field 9 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf9_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf9_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26032).add(128 * n).cast()
            })
    }
    ///0x65b0 - Common FIFO Access Data Field 9 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf90_1(&self) -> &Cfdcfdf9_1 {
        self.cfdcfdf9_1(0)
    }
    ///0x6630 - Common FIFO Access Data Field 9 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf91_1(&self) -> &Cfdcfdf9_1 {
        self.cfdcfdf9_1(1)
    }
    ///0x66b0 - Common FIFO Access Data Field 9 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf92_1(&self) -> &Cfdcfdf9_1 {
        self.cfdcfdf9_1(2)
    }
    ///0x65b4..0x65c0 - Common FIFO Access Data Field 10 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf10_1(&self, n: usize) -> &Cfdcfdf10_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26036).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65b4..0x65c0 - Common FIFO Access Data Field 10 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf10_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf10_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26036).add(128 * n).cast()
            })
    }
    ///0x65b4 - Common FIFO Access Data Field 10 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf100_1(&self) -> &Cfdcfdf10_1 {
        self.cfdcfdf10_1(0)
    }
    ///0x6634 - Common FIFO Access Data Field 10 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf101_1(&self) -> &Cfdcfdf10_1 {
        self.cfdcfdf10_1(1)
    }
    ///0x66b4 - Common FIFO Access Data Field 10 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf102_1(&self) -> &Cfdcfdf10_1 {
        self.cfdcfdf10_1(2)
    }
    ///0x65b8..0x65c4 - Common FIFO Access Data Field 11 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf11_1(&self, n: usize) -> &Cfdcfdf11_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26040).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65b8..0x65c4 - Common FIFO Access Data Field 11 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf11_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf11_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26040).add(128 * n).cast()
            })
    }
    ///0x65b8 - Common FIFO Access Data Field 11 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf110_1(&self) -> &Cfdcfdf11_1 {
        self.cfdcfdf11_1(0)
    }
    ///0x6638 - Common FIFO Access Data Field 11 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf111_1(&self) -> &Cfdcfdf11_1 {
        self.cfdcfdf11_1(1)
    }
    ///0x66b8 - Common FIFO Access Data Field 11 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf112_1(&self) -> &Cfdcfdf11_1 {
        self.cfdcfdf11_1(2)
    }
    ///0x65bc..0x65c8 - Common FIFO Access Data Field 12 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf12_1(&self, n: usize) -> &Cfdcfdf12_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26044).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65bc..0x65c8 - Common FIFO Access Data Field 12 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf12_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf12_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26044).add(128 * n).cast()
            })
    }
    ///0x65bc - Common FIFO Access Data Field 12 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf120_1(&self) -> &Cfdcfdf12_1 {
        self.cfdcfdf12_1(0)
    }
    ///0x663c - Common FIFO Access Data Field 12 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf121_1(&self) -> &Cfdcfdf12_1 {
        self.cfdcfdf12_1(1)
    }
    ///0x66bc - Common FIFO Access Data Field 12 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf122_1(&self) -> &Cfdcfdf12_1 {
        self.cfdcfdf12_1(2)
    }
    ///0x65c0..0x65cc - Common FIFO Access Data Field 13 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf13_1(&self, n: usize) -> &Cfdcfdf13_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26048).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65c0..0x65cc - Common FIFO Access Data Field 13 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf13_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf13_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26048).add(128 * n).cast()
            })
    }
    ///0x65c0 - Common FIFO Access Data Field 13 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf130_1(&self) -> &Cfdcfdf13_1 {
        self.cfdcfdf13_1(0)
    }
    ///0x6640 - Common FIFO Access Data Field 13 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf131_1(&self) -> &Cfdcfdf13_1 {
        self.cfdcfdf13_1(1)
    }
    ///0x66c0 - Common FIFO Access Data Field 13 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf132_1(&self) -> &Cfdcfdf13_1 {
        self.cfdcfdf13_1(2)
    }
    ///0x65c4..0x65d0 - Common FIFO Access Data Field 14 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf14_1(&self, n: usize) -> &Cfdcfdf14_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26052).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65c4..0x65d0 - Common FIFO Access Data Field 14 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf14_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf14_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26052).add(128 * n).cast()
            })
    }
    ///0x65c4 - Common FIFO Access Data Field 14 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf140_1(&self) -> &Cfdcfdf14_1 {
        self.cfdcfdf14_1(0)
    }
    ///0x6644 - Common FIFO Access Data Field 14 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf141_1(&self) -> &Cfdcfdf14_1 {
        self.cfdcfdf14_1(1)
    }
    ///0x66c4 - Common FIFO Access Data Field 14 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf142_1(&self) -> &Cfdcfdf14_1 {
        self.cfdcfdf14_1(2)
    }
    ///0x65c8..0x65d4 - Common FIFO Access Data Field 15 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdcfdf15_1(&self, n: usize) -> &Cfdcfdf15_1 {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(26056).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x65c8..0x65d4 - Common FIFO Access Data Field 15 Register %s Channel 1
    #[inline(always)]
    pub fn cfdcfdf15_1_iter(&self) -> impl Iterator<Item = &Cfdcfdf15_1> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(26056).add(128 * n).cast()
            })
    }
    ///0x65c8 - Common FIFO Access Data Field 15 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf150_1(&self) -> &Cfdcfdf15_1 {
        self.cfdcfdf15_1(0)
    }
    ///0x6648 - Common FIFO Access Data Field 15 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf151_1(&self) -> &Cfdcfdf15_1 {
        self.cfdcfdf15_1(1)
    }
    ///0x66c8 - Common FIFO Access Data Field 15 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdcfdf152_1(&self) -> &Cfdcfdf15_1 {
        self.cfdcfdf15_1(2)
    }
    ///0x8000..0x8008 - TX History List Access Registers 0
    #[inline(always)]
    pub const fn cfdthlacc0(&self, n: usize) -> &Cfdthlacc0 {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32768).add(8 * n).cast() }
    }
    ///Iterator for array of:
    ///0x8000..0x8008 - TX History List Access Registers 0
    #[inline(always)]
    pub fn cfdthlacc0_iter(&self) -> impl Iterator<Item = &Cfdthlacc0> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(32768).add(8 * n).cast()
            })
    }
    ///0x8004..0x800c - TX History List Access Registers 1
    #[inline(always)]
    pub const fn cfdthlacc1(&self, n: usize) -> &Cfdthlacc1 {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32772).add(8 * n).cast() }
    }
    ///Iterator for array of:
    ///0x8004..0x800c - TX History List Access Registers 1
    #[inline(always)]
    pub fn cfdthlacc1_iter(&self) -> impl Iterator<Item = &Cfdthlacc1> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(32772).add(8 * n).cast()
            })
    }
    ///0x8400..0x8500 - RAM Test Page Access Registers %s
    #[inline(always)]
    pub const fn cfdrpgacc(&self, n: usize) -> &Cfdrpgacc {
        &self.cfdrpgacc[n]
    }
    ///Iterator for array of:
    ///0x8400..0x8500 - RAM Test Page Access Registers %s
    #[inline(always)]
    pub fn cfdrpgacc_iter(&self) -> impl Iterator<Item = &Cfdrpgacc> {
        self.cfdrpgacc.iter()
    }
    ///0x10000..0x10020 - TX Message Buffer ID Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmid_0(&self, n: usize) -> &Cfdtmid0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65536).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10000..0x10020 - TX Message Buffer ID Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmid_0_iter(&self) -> impl Iterator<Item = &Cfdtmid0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65536).add(128 * n).cast()
            })
    }
    ///0x10000 - TX Message Buffer ID Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmid0_0(&self) -> &Cfdtmid0 {
        self.cfdtmid_0(0)
    }
    ///0x10080 - TX Message Buffer ID Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmid1_0(&self) -> &Cfdtmid0 {
        self.cfdtmid_0(1)
    }
    ///0x10100 - TX Message Buffer ID Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmid2_0(&self) -> &Cfdtmid0 {
        self.cfdtmid_0(2)
    }
    ///0x10180 - TX Message Buffer ID Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmid3_0(&self) -> &Cfdtmid0 {
        self.cfdtmid_0(3)
    }
    ///0x10200 - TX Message Buffer ID Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmid4_0(&self) -> &Cfdtmid0 {
        self.cfdtmid_0(4)
    }
    ///0x10280 - TX Message Buffer ID Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmid5_0(&self) -> &Cfdtmid0 {
        self.cfdtmid_0(5)
    }
    ///0x10300 - TX Message Buffer ID Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmid6_0(&self) -> &Cfdtmid0 {
        self.cfdtmid_0(6)
    }
    ///0x10380 - TX Message Buffer ID Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmid7_0(&self) -> &Cfdtmid0 {
        self.cfdtmid_0(7)
    }
    ///0x10004..0x10024 - TX Message Buffer Pointer Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmptr_0(&self, n: usize) -> &Cfdtmptr0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65540).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10004..0x10024 - TX Message Buffer Pointer Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmptr_0_iter(&self) -> impl Iterator<Item = &Cfdtmptr0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65540).add(128 * n).cast()
            })
    }
    ///0x10004 - TX Message Buffer Pointer Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr0_0(&self) -> &Cfdtmptr0 {
        self.cfdtmptr_0(0)
    }
    ///0x10084 - TX Message Buffer Pointer Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr1_0(&self) -> &Cfdtmptr0 {
        self.cfdtmptr_0(1)
    }
    ///0x10104 - TX Message Buffer Pointer Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr2_0(&self) -> &Cfdtmptr0 {
        self.cfdtmptr_0(2)
    }
    ///0x10184 - TX Message Buffer Pointer Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr3_0(&self) -> &Cfdtmptr0 {
        self.cfdtmptr_0(3)
    }
    ///0x10204 - TX Message Buffer Pointer Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr4_0(&self) -> &Cfdtmptr0 {
        self.cfdtmptr_0(4)
    }
    ///0x10284 - TX Message Buffer Pointer Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr5_0(&self) -> &Cfdtmptr0 {
        self.cfdtmptr_0(5)
    }
    ///0x10304 - TX Message Buffer Pointer Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr6_0(&self) -> &Cfdtmptr0 {
        self.cfdtmptr_0(6)
    }
    ///0x10384 - TX Message Buffer Pointer Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr7_0(&self) -> &Cfdtmptr0 {
        self.cfdtmptr_0(7)
    }
    ///0x10008..0x10028 - TX Message Buffer CANFD Control Register %s Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr_0(&self, n: usize) -> &Cfdtmfdctr0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65544).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10008..0x10028 - TX Message Buffer CANFD Control Register %s Channel i
    #[inline(always)]
    pub fn cfdtmfdctr_0_iter(&self) -> impl Iterator<Item = &Cfdtmfdctr0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65544).add(128 * n).cast()
            })
    }
    ///0x10008 - TX Message Buffer CANFD Control Register 0 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr0_0(&self) -> &Cfdtmfdctr0 {
        self.cfdtmfdctr_0(0)
    }
    ///0x10088 - TX Message Buffer CANFD Control Register 1 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr1_0(&self) -> &Cfdtmfdctr0 {
        self.cfdtmfdctr_0(1)
    }
    ///0x10108 - TX Message Buffer CANFD Control Register 2 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr2_0(&self) -> &Cfdtmfdctr0 {
        self.cfdtmfdctr_0(2)
    }
    ///0x10188 - TX Message Buffer CANFD Control Register 3 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr3_0(&self) -> &Cfdtmfdctr0 {
        self.cfdtmfdctr_0(3)
    }
    ///0x10208 - TX Message Buffer CANFD Control Register 4 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr4_0(&self) -> &Cfdtmfdctr0 {
        self.cfdtmfdctr_0(4)
    }
    ///0x10288 - TX Message Buffer CANFD Control Register 5 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr5_0(&self) -> &Cfdtmfdctr0 {
        self.cfdtmfdctr_0(5)
    }
    ///0x10308 - TX Message Buffer CANFD Control Register 6 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr6_0(&self) -> &Cfdtmfdctr0 {
        self.cfdtmfdctr_0(6)
    }
    ///0x10388 - TX Message Buffer CANFD Control Register 7 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr7_0(&self) -> &Cfdtmfdctr0 {
        self.cfdtmfdctr_0(7)
    }
    ///0x1000c..0x1002c - TX Message Buffer Data Field 0 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0__0(&self, n: usize) -> &Cfdtmdf0_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65548).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x1000c..0x1002c - TX Message Buffer Data Field 0 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf0__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf0_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65548).add(128 * n).cast()
            })
    }
    ///0x1000c - TX Message Buffer Data Field 0 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_0_0(&self) -> &Cfdtmdf0_0 {
        self.cfdtmdf0__0(0)
    }
    ///0x1008c - TX Message Buffer Data Field 0 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_1_0(&self) -> &Cfdtmdf0_0 {
        self.cfdtmdf0__0(1)
    }
    ///0x1010c - TX Message Buffer Data Field 0 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_2_0(&self) -> &Cfdtmdf0_0 {
        self.cfdtmdf0__0(2)
    }
    ///0x1018c - TX Message Buffer Data Field 0 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_3_0(&self) -> &Cfdtmdf0_0 {
        self.cfdtmdf0__0(3)
    }
    ///0x1020c - TX Message Buffer Data Field 0 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_4_0(&self) -> &Cfdtmdf0_0 {
        self.cfdtmdf0__0(4)
    }
    ///0x1028c - TX Message Buffer Data Field 0 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_5_0(&self) -> &Cfdtmdf0_0 {
        self.cfdtmdf0__0(5)
    }
    ///0x1030c - TX Message Buffer Data Field 0 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_6_0(&self) -> &Cfdtmdf0_0 {
        self.cfdtmdf0__0(6)
    }
    ///0x1038c - TX Message Buffer Data Field 0 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_7_0(&self) -> &Cfdtmdf0_0 {
        self.cfdtmdf0__0(7)
    }
    ///0x10010..0x10030 - TX Message Buffer Data Field 1 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1__0(&self, n: usize) -> &Cfdtmdf1_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65552).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10010..0x10030 - TX Message Buffer Data Field 1 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf1__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf1_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65552).add(128 * n).cast()
            })
    }
    ///0x10010 - TX Message Buffer Data Field 1 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_0_0(&self) -> &Cfdtmdf1_0 {
        self.cfdtmdf1__0(0)
    }
    ///0x10090 - TX Message Buffer Data Field 1 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_1_0(&self) -> &Cfdtmdf1_0 {
        self.cfdtmdf1__0(1)
    }
    ///0x10110 - TX Message Buffer Data Field 1 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_2_0(&self) -> &Cfdtmdf1_0 {
        self.cfdtmdf1__0(2)
    }
    ///0x10190 - TX Message Buffer Data Field 1 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_3_0(&self) -> &Cfdtmdf1_0 {
        self.cfdtmdf1__0(3)
    }
    ///0x10210 - TX Message Buffer Data Field 1 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_4_0(&self) -> &Cfdtmdf1_0 {
        self.cfdtmdf1__0(4)
    }
    ///0x10290 - TX Message Buffer Data Field 1 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_5_0(&self) -> &Cfdtmdf1_0 {
        self.cfdtmdf1__0(5)
    }
    ///0x10310 - TX Message Buffer Data Field 1 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_6_0(&self) -> &Cfdtmdf1_0 {
        self.cfdtmdf1__0(6)
    }
    ///0x10390 - TX Message Buffer Data Field 1 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_7_0(&self) -> &Cfdtmdf1_0 {
        self.cfdtmdf1__0(7)
    }
    ///0x10014..0x10034 - TX Message Buffer Data Field 2 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2__0(&self, n: usize) -> &Cfdtmdf2_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65556).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10014..0x10034 - TX Message Buffer Data Field 2 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf2__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf2_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65556).add(128 * n).cast()
            })
    }
    ///0x10014 - TX Message Buffer Data Field 2 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_0_0(&self) -> &Cfdtmdf2_0 {
        self.cfdtmdf2__0(0)
    }
    ///0x10094 - TX Message Buffer Data Field 2 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_1_0(&self) -> &Cfdtmdf2_0 {
        self.cfdtmdf2__0(1)
    }
    ///0x10114 - TX Message Buffer Data Field 2 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_2_0(&self) -> &Cfdtmdf2_0 {
        self.cfdtmdf2__0(2)
    }
    ///0x10194 - TX Message Buffer Data Field 2 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_3_0(&self) -> &Cfdtmdf2_0 {
        self.cfdtmdf2__0(3)
    }
    ///0x10214 - TX Message Buffer Data Field 2 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_4_0(&self) -> &Cfdtmdf2_0 {
        self.cfdtmdf2__0(4)
    }
    ///0x10294 - TX Message Buffer Data Field 2 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_5_0(&self) -> &Cfdtmdf2_0 {
        self.cfdtmdf2__0(5)
    }
    ///0x10314 - TX Message Buffer Data Field 2 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_6_0(&self) -> &Cfdtmdf2_0 {
        self.cfdtmdf2__0(6)
    }
    ///0x10394 - TX Message Buffer Data Field 2 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_7_0(&self) -> &Cfdtmdf2_0 {
        self.cfdtmdf2__0(7)
    }
    ///0x10018..0x10038 - TX Message Buffer Data Field 3 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3__0(&self, n: usize) -> &Cfdtmdf3_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65560).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10018..0x10038 - TX Message Buffer Data Field 3 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf3__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf3_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65560).add(128 * n).cast()
            })
    }
    ///0x10018 - TX Message Buffer Data Field 3 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_0_0(&self) -> &Cfdtmdf3_0 {
        self.cfdtmdf3__0(0)
    }
    ///0x10098 - TX Message Buffer Data Field 3 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_1_0(&self) -> &Cfdtmdf3_0 {
        self.cfdtmdf3__0(1)
    }
    ///0x10118 - TX Message Buffer Data Field 3 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_2_0(&self) -> &Cfdtmdf3_0 {
        self.cfdtmdf3__0(2)
    }
    ///0x10198 - TX Message Buffer Data Field 3 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_3_0(&self) -> &Cfdtmdf3_0 {
        self.cfdtmdf3__0(3)
    }
    ///0x10218 - TX Message Buffer Data Field 3 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_4_0(&self) -> &Cfdtmdf3_0 {
        self.cfdtmdf3__0(4)
    }
    ///0x10298 - TX Message Buffer Data Field 3 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_5_0(&self) -> &Cfdtmdf3_0 {
        self.cfdtmdf3__0(5)
    }
    ///0x10318 - TX Message Buffer Data Field 3 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_6_0(&self) -> &Cfdtmdf3_0 {
        self.cfdtmdf3__0(6)
    }
    ///0x10398 - TX Message Buffer Data Field 3 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_7_0(&self) -> &Cfdtmdf3_0 {
        self.cfdtmdf3__0(7)
    }
    ///0x1001c..0x1003c - TX Message Buffer Data Field 4 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4__0(&self, n: usize) -> &Cfdtmdf4_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65564).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x1001c..0x1003c - TX Message Buffer Data Field 4 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf4__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf4_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65564).add(128 * n).cast()
            })
    }
    ///0x1001c - TX Message Buffer Data Field 4 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_0_0(&self) -> &Cfdtmdf4_0 {
        self.cfdtmdf4__0(0)
    }
    ///0x1009c - TX Message Buffer Data Field 4 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_1_0(&self) -> &Cfdtmdf4_0 {
        self.cfdtmdf4__0(1)
    }
    ///0x1011c - TX Message Buffer Data Field 4 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_2_0(&self) -> &Cfdtmdf4_0 {
        self.cfdtmdf4__0(2)
    }
    ///0x1019c - TX Message Buffer Data Field 4 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_3_0(&self) -> &Cfdtmdf4_0 {
        self.cfdtmdf4__0(3)
    }
    ///0x1021c - TX Message Buffer Data Field 4 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_4_0(&self) -> &Cfdtmdf4_0 {
        self.cfdtmdf4__0(4)
    }
    ///0x1029c - TX Message Buffer Data Field 4 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_5_0(&self) -> &Cfdtmdf4_0 {
        self.cfdtmdf4__0(5)
    }
    ///0x1031c - TX Message Buffer Data Field 4 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_6_0(&self) -> &Cfdtmdf4_0 {
        self.cfdtmdf4__0(6)
    }
    ///0x1039c - TX Message Buffer Data Field 4 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_7_0(&self) -> &Cfdtmdf4_0 {
        self.cfdtmdf4__0(7)
    }
    ///0x10020..0x10040 - TX Message Buffer Data Field 5 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5__0(&self, n: usize) -> &Cfdtmdf5_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65568).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10020..0x10040 - TX Message Buffer Data Field 5 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf5__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf5_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65568).add(128 * n).cast()
            })
    }
    ///0x10020 - TX Message Buffer Data Field 5 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_0_0(&self) -> &Cfdtmdf5_0 {
        self.cfdtmdf5__0(0)
    }
    ///0x100a0 - TX Message Buffer Data Field 5 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_1_0(&self) -> &Cfdtmdf5_0 {
        self.cfdtmdf5__0(1)
    }
    ///0x10120 - TX Message Buffer Data Field 5 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_2_0(&self) -> &Cfdtmdf5_0 {
        self.cfdtmdf5__0(2)
    }
    ///0x101a0 - TX Message Buffer Data Field 5 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_3_0(&self) -> &Cfdtmdf5_0 {
        self.cfdtmdf5__0(3)
    }
    ///0x10220 - TX Message Buffer Data Field 5 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_4_0(&self) -> &Cfdtmdf5_0 {
        self.cfdtmdf5__0(4)
    }
    ///0x102a0 - TX Message Buffer Data Field 5 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_5_0(&self) -> &Cfdtmdf5_0 {
        self.cfdtmdf5__0(5)
    }
    ///0x10320 - TX Message Buffer Data Field 5 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_6_0(&self) -> &Cfdtmdf5_0 {
        self.cfdtmdf5__0(6)
    }
    ///0x103a0 - TX Message Buffer Data Field 5 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_7_0(&self) -> &Cfdtmdf5_0 {
        self.cfdtmdf5__0(7)
    }
    ///0x10024..0x10044 - TX Message Buffer Data Field 6 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6__0(&self, n: usize) -> &Cfdtmdf6_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65572).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10024..0x10044 - TX Message Buffer Data Field 6 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf6__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf6_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65572).add(128 * n).cast()
            })
    }
    ///0x10024 - TX Message Buffer Data Field 6 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_0_0(&self) -> &Cfdtmdf6_0 {
        self.cfdtmdf6__0(0)
    }
    ///0x100a4 - TX Message Buffer Data Field 6 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_1_0(&self) -> &Cfdtmdf6_0 {
        self.cfdtmdf6__0(1)
    }
    ///0x10124 - TX Message Buffer Data Field 6 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_2_0(&self) -> &Cfdtmdf6_0 {
        self.cfdtmdf6__0(2)
    }
    ///0x101a4 - TX Message Buffer Data Field 6 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_3_0(&self) -> &Cfdtmdf6_0 {
        self.cfdtmdf6__0(3)
    }
    ///0x10224 - TX Message Buffer Data Field 6 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_4_0(&self) -> &Cfdtmdf6_0 {
        self.cfdtmdf6__0(4)
    }
    ///0x102a4 - TX Message Buffer Data Field 6 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_5_0(&self) -> &Cfdtmdf6_0 {
        self.cfdtmdf6__0(5)
    }
    ///0x10324 - TX Message Buffer Data Field 6 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_6_0(&self) -> &Cfdtmdf6_0 {
        self.cfdtmdf6__0(6)
    }
    ///0x103a4 - TX Message Buffer Data Field 6 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_7_0(&self) -> &Cfdtmdf6_0 {
        self.cfdtmdf6__0(7)
    }
    ///0x10028..0x10048 - TX Message Buffer Data Field 7 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7__0(&self, n: usize) -> &Cfdtmdf7_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65576).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10028..0x10048 - TX Message Buffer Data Field 7 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf7__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf7_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65576).add(128 * n).cast()
            })
    }
    ///0x10028 - TX Message Buffer Data Field 7 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_0_0(&self) -> &Cfdtmdf7_0 {
        self.cfdtmdf7__0(0)
    }
    ///0x100a8 - TX Message Buffer Data Field 7 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_1_0(&self) -> &Cfdtmdf7_0 {
        self.cfdtmdf7__0(1)
    }
    ///0x10128 - TX Message Buffer Data Field 7 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_2_0(&self) -> &Cfdtmdf7_0 {
        self.cfdtmdf7__0(2)
    }
    ///0x101a8 - TX Message Buffer Data Field 7 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_3_0(&self) -> &Cfdtmdf7_0 {
        self.cfdtmdf7__0(3)
    }
    ///0x10228 - TX Message Buffer Data Field 7 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_4_0(&self) -> &Cfdtmdf7_0 {
        self.cfdtmdf7__0(4)
    }
    ///0x102a8 - TX Message Buffer Data Field 7 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_5_0(&self) -> &Cfdtmdf7_0 {
        self.cfdtmdf7__0(5)
    }
    ///0x10328 - TX Message Buffer Data Field 7 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_6_0(&self) -> &Cfdtmdf7_0 {
        self.cfdtmdf7__0(6)
    }
    ///0x103a8 - TX Message Buffer Data Field 7 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_7_0(&self) -> &Cfdtmdf7_0 {
        self.cfdtmdf7__0(7)
    }
    ///0x1002c..0x1004c - TX Message Buffer Data Field 8 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8__0(&self, n: usize) -> &Cfdtmdf8_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65580).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x1002c..0x1004c - TX Message Buffer Data Field 8 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf8__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf8_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65580).add(128 * n).cast()
            })
    }
    ///0x1002c - TX Message Buffer Data Field 8 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_0_0(&self) -> &Cfdtmdf8_0 {
        self.cfdtmdf8__0(0)
    }
    ///0x100ac - TX Message Buffer Data Field 8 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_1_0(&self) -> &Cfdtmdf8_0 {
        self.cfdtmdf8__0(1)
    }
    ///0x1012c - TX Message Buffer Data Field 8 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_2_0(&self) -> &Cfdtmdf8_0 {
        self.cfdtmdf8__0(2)
    }
    ///0x101ac - TX Message Buffer Data Field 8 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_3_0(&self) -> &Cfdtmdf8_0 {
        self.cfdtmdf8__0(3)
    }
    ///0x1022c - TX Message Buffer Data Field 8 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_4_0(&self) -> &Cfdtmdf8_0 {
        self.cfdtmdf8__0(4)
    }
    ///0x102ac - TX Message Buffer Data Field 8 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_5_0(&self) -> &Cfdtmdf8_0 {
        self.cfdtmdf8__0(5)
    }
    ///0x1032c - TX Message Buffer Data Field 8 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_6_0(&self) -> &Cfdtmdf8_0 {
        self.cfdtmdf8__0(6)
    }
    ///0x103ac - TX Message Buffer Data Field 8 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_7_0(&self) -> &Cfdtmdf8_0 {
        self.cfdtmdf8__0(7)
    }
    ///0x10030..0x10050 - TX Message Buffer Data Field 9 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9__0(&self, n: usize) -> &Cfdtmdf9_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65584).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10030..0x10050 - TX Message Buffer Data Field 9 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf9__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf9_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65584).add(128 * n).cast()
            })
    }
    ///0x10030 - TX Message Buffer Data Field 9 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_0_0(&self) -> &Cfdtmdf9_0 {
        self.cfdtmdf9__0(0)
    }
    ///0x100b0 - TX Message Buffer Data Field 9 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_1_0(&self) -> &Cfdtmdf9_0 {
        self.cfdtmdf9__0(1)
    }
    ///0x10130 - TX Message Buffer Data Field 9 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_2_0(&self) -> &Cfdtmdf9_0 {
        self.cfdtmdf9__0(2)
    }
    ///0x101b0 - TX Message Buffer Data Field 9 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_3_0(&self) -> &Cfdtmdf9_0 {
        self.cfdtmdf9__0(3)
    }
    ///0x10230 - TX Message Buffer Data Field 9 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_4_0(&self) -> &Cfdtmdf9_0 {
        self.cfdtmdf9__0(4)
    }
    ///0x102b0 - TX Message Buffer Data Field 9 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_5_0(&self) -> &Cfdtmdf9_0 {
        self.cfdtmdf9__0(5)
    }
    ///0x10330 - TX Message Buffer Data Field 9 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_6_0(&self) -> &Cfdtmdf9_0 {
        self.cfdtmdf9__0(6)
    }
    ///0x103b0 - TX Message Buffer Data Field 9 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_7_0(&self) -> &Cfdtmdf9_0 {
        self.cfdtmdf9__0(7)
    }
    ///0x10034..0x10054 - TX Message Buffer Data Field 10 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10__0(&self, n: usize) -> &Cfdtmdf10_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65588).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10034..0x10054 - TX Message Buffer Data Field 10 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf10__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf10_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65588).add(128 * n).cast()
            })
    }
    ///0x10034 - TX Message Buffer Data Field 10 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_0_0(&self) -> &Cfdtmdf10_0 {
        self.cfdtmdf10__0(0)
    }
    ///0x100b4 - TX Message Buffer Data Field 10 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_1_0(&self) -> &Cfdtmdf10_0 {
        self.cfdtmdf10__0(1)
    }
    ///0x10134 - TX Message Buffer Data Field 10 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_2_0(&self) -> &Cfdtmdf10_0 {
        self.cfdtmdf10__0(2)
    }
    ///0x101b4 - TX Message Buffer Data Field 10 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_3_0(&self) -> &Cfdtmdf10_0 {
        self.cfdtmdf10__0(3)
    }
    ///0x10234 - TX Message Buffer Data Field 10 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_4_0(&self) -> &Cfdtmdf10_0 {
        self.cfdtmdf10__0(4)
    }
    ///0x102b4 - TX Message Buffer Data Field 10 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_5_0(&self) -> &Cfdtmdf10_0 {
        self.cfdtmdf10__0(5)
    }
    ///0x10334 - TX Message Buffer Data Field 10 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_6_0(&self) -> &Cfdtmdf10_0 {
        self.cfdtmdf10__0(6)
    }
    ///0x103b4 - TX Message Buffer Data Field 10 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_7_0(&self) -> &Cfdtmdf10_0 {
        self.cfdtmdf10__0(7)
    }
    ///0x10038..0x10058 - TX Message Buffer Data Field 11 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11__0(&self, n: usize) -> &Cfdtmdf11_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65592).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10038..0x10058 - TX Message Buffer Data Field 11 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf11__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf11_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65592).add(128 * n).cast()
            })
    }
    ///0x10038 - TX Message Buffer Data Field 11 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_0_0(&self) -> &Cfdtmdf11_0 {
        self.cfdtmdf11__0(0)
    }
    ///0x100b8 - TX Message Buffer Data Field 11 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_1_0(&self) -> &Cfdtmdf11_0 {
        self.cfdtmdf11__0(1)
    }
    ///0x10138 - TX Message Buffer Data Field 11 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_2_0(&self) -> &Cfdtmdf11_0 {
        self.cfdtmdf11__0(2)
    }
    ///0x101b8 - TX Message Buffer Data Field 11 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_3_0(&self) -> &Cfdtmdf11_0 {
        self.cfdtmdf11__0(3)
    }
    ///0x10238 - TX Message Buffer Data Field 11 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_4_0(&self) -> &Cfdtmdf11_0 {
        self.cfdtmdf11__0(4)
    }
    ///0x102b8 - TX Message Buffer Data Field 11 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_5_0(&self) -> &Cfdtmdf11_0 {
        self.cfdtmdf11__0(5)
    }
    ///0x10338 - TX Message Buffer Data Field 11 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_6_0(&self) -> &Cfdtmdf11_0 {
        self.cfdtmdf11__0(6)
    }
    ///0x103b8 - TX Message Buffer Data Field 11 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_7_0(&self) -> &Cfdtmdf11_0 {
        self.cfdtmdf11__0(7)
    }
    ///0x1003c..0x1005c - TX Message Buffer Data Field 12 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12__0(&self, n: usize) -> &Cfdtmdf12_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65596).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x1003c..0x1005c - TX Message Buffer Data Field 12 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf12__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf12_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65596).add(128 * n).cast()
            })
    }
    ///0x1003c - TX Message Buffer Data Field 12 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_0_0(&self) -> &Cfdtmdf12_0 {
        self.cfdtmdf12__0(0)
    }
    ///0x100bc - TX Message Buffer Data Field 12 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_1_0(&self) -> &Cfdtmdf12_0 {
        self.cfdtmdf12__0(1)
    }
    ///0x1013c - TX Message Buffer Data Field 12 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_2_0(&self) -> &Cfdtmdf12_0 {
        self.cfdtmdf12__0(2)
    }
    ///0x101bc - TX Message Buffer Data Field 12 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_3_0(&self) -> &Cfdtmdf12_0 {
        self.cfdtmdf12__0(3)
    }
    ///0x1023c - TX Message Buffer Data Field 12 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_4_0(&self) -> &Cfdtmdf12_0 {
        self.cfdtmdf12__0(4)
    }
    ///0x102bc - TX Message Buffer Data Field 12 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_5_0(&self) -> &Cfdtmdf12_0 {
        self.cfdtmdf12__0(5)
    }
    ///0x1033c - TX Message Buffer Data Field 12 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_6_0(&self) -> &Cfdtmdf12_0 {
        self.cfdtmdf12__0(6)
    }
    ///0x103bc - TX Message Buffer Data Field 12 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_7_0(&self) -> &Cfdtmdf12_0 {
        self.cfdtmdf12__0(7)
    }
    ///0x10040..0x10060 - TX Message Buffer Data Field 13 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13__0(&self, n: usize) -> &Cfdtmdf13_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65600).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10040..0x10060 - TX Message Buffer Data Field 13 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf13__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf13_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65600).add(128 * n).cast()
            })
    }
    ///0x10040 - TX Message Buffer Data Field 13 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_0_0(&self) -> &Cfdtmdf13_0 {
        self.cfdtmdf13__0(0)
    }
    ///0x100c0 - TX Message Buffer Data Field 13 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_1_0(&self) -> &Cfdtmdf13_0 {
        self.cfdtmdf13__0(1)
    }
    ///0x10140 - TX Message Buffer Data Field 13 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_2_0(&self) -> &Cfdtmdf13_0 {
        self.cfdtmdf13__0(2)
    }
    ///0x101c0 - TX Message Buffer Data Field 13 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_3_0(&self) -> &Cfdtmdf13_0 {
        self.cfdtmdf13__0(3)
    }
    ///0x10240 - TX Message Buffer Data Field 13 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_4_0(&self) -> &Cfdtmdf13_0 {
        self.cfdtmdf13__0(4)
    }
    ///0x102c0 - TX Message Buffer Data Field 13 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_5_0(&self) -> &Cfdtmdf13_0 {
        self.cfdtmdf13__0(5)
    }
    ///0x10340 - TX Message Buffer Data Field 13 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_6_0(&self) -> &Cfdtmdf13_0 {
        self.cfdtmdf13__0(6)
    }
    ///0x103c0 - TX Message Buffer Data Field 13 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_7_0(&self) -> &Cfdtmdf13_0 {
        self.cfdtmdf13__0(7)
    }
    ///0x10044..0x10064 - TX Message Buffer Data Field 14 Register %s Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14__0(&self, n: usize) -> &Cfdtmdf14_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65604).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10044..0x10064 - TX Message Buffer Data Field 14 Register %s Channel 0
    #[inline(always)]
    pub fn cfdtmdf14__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf14_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65604).add(128 * n).cast()
            })
    }
    ///0x10044 - TX Message Buffer Data Field 14 Register 0 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_0_0(&self) -> &Cfdtmdf14_0 {
        self.cfdtmdf14__0(0)
    }
    ///0x100c4 - TX Message Buffer Data Field 14 Register 1 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_1_0(&self) -> &Cfdtmdf14_0 {
        self.cfdtmdf14__0(1)
    }
    ///0x10144 - TX Message Buffer Data Field 14 Register 2 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_2_0(&self) -> &Cfdtmdf14_0 {
        self.cfdtmdf14__0(2)
    }
    ///0x101c4 - TX Message Buffer Data Field 14 Register 3 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_3_0(&self) -> &Cfdtmdf14_0 {
        self.cfdtmdf14__0(3)
    }
    ///0x10244 - TX Message Buffer Data Field 14 Register 4 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_4_0(&self) -> &Cfdtmdf14_0 {
        self.cfdtmdf14__0(4)
    }
    ///0x102c4 - TX Message Buffer Data Field 14 Register 5 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_5_0(&self) -> &Cfdtmdf14_0 {
        self.cfdtmdf14__0(5)
    }
    ///0x10344 - TX Message Buffer Data Field 14 Register 6 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_6_0(&self) -> &Cfdtmdf14_0 {
        self.cfdtmdf14__0(6)
    }
    ///0x103c4 - TX Message Buffer Data Field 14 Register 7 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_7_0(&self) -> &Cfdtmdf14_0 {
        self.cfdtmdf14__0(7)
    }
    ///0x10048..0x10068 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15__0(&self, n: usize) -> &Cfdtmdf15_0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(65608).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x10048..0x10068 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub fn cfdtmdf15__0_iter(&self) -> impl Iterator<Item = &Cfdtmdf15_0> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(65608).add(128 * n).cast()
            })
    }
    ///0x10048 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_0_0(&self) -> &Cfdtmdf15_0 {
        self.cfdtmdf15__0(0)
    }
    ///0x100c8 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_1_0(&self) -> &Cfdtmdf15_0 {
        self.cfdtmdf15__0(1)
    }
    ///0x10148 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_2_0(&self) -> &Cfdtmdf15_0 {
        self.cfdtmdf15__0(2)
    }
    ///0x101c8 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_3_0(&self) -> &Cfdtmdf15_0 {
        self.cfdtmdf15__0(3)
    }
    ///0x10248 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_4_0(&self) -> &Cfdtmdf15_0 {
        self.cfdtmdf15__0(4)
    }
    ///0x102c8 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_5_0(&self) -> &Cfdtmdf15_0 {
        self.cfdtmdf15__0(5)
    }
    ///0x10348 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_6_0(&self) -> &Cfdtmdf15_0 {
        self.cfdtmdf15__0(6)
    }
    ///0x103c8 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_7_0(&self) -> &Cfdtmdf15_0 {
        self.cfdtmdf15__0(7)
    }
    ///0x11000 - TX Message Buffer ID Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmid32_0(&self) -> &Cfdtmid0 {
        &self.cfdtmid32_0
    }
    ///0x11004 - TX Message Buffer Pointer Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr32_0(&self) -> &Cfdtmptr0 {
        &self.cfdtmptr32_0
    }
    ///0x11008 - TX Message Buffer CANFD Control Register 32 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr32_0(&self) -> &Cfdtmfdctr0 {
        &self.cfdtmfdctr32_0
    }
    ///0x1100c - TX Message Buffer Data Field 0 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_32_0(&self) -> &Cfdtmdf0_0 {
        &self.cfdtmdf0_32_0
    }
    ///0x11010 - TX Message Buffer Data Field 1 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_32_0(&self) -> &Cfdtmdf1_0 {
        &self.cfdtmdf1_32_0
    }
    ///0x11014 - TX Message Buffer Data Field 2 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_32_0(&self) -> &Cfdtmdf2_0 {
        &self.cfdtmdf2_32_0
    }
    ///0x11018 - TX Message Buffer Data Field 3 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_32_0(&self) -> &Cfdtmdf3_0 {
        &self.cfdtmdf3_32_0
    }
    ///0x1101c - TX Message Buffer Data Field 4 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_32_0(&self) -> &Cfdtmdf4_0 {
        &self.cfdtmdf4_32_0
    }
    ///0x11020 - TX Message Buffer Data Field 5 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_32_0(&self) -> &Cfdtmdf5_0 {
        &self.cfdtmdf5_32_0
    }
    ///0x11024 - TX Message Buffer Data Field 6 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_32_0(&self) -> &Cfdtmdf6_0 {
        &self.cfdtmdf6_32_0
    }
    ///0x11028 - TX Message Buffer Data Field 7 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_32_0(&self) -> &Cfdtmdf7_0 {
        &self.cfdtmdf7_32_0
    }
    ///0x1102c - TX Message Buffer Data Field 8 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_32_0(&self) -> &Cfdtmdf8_0 {
        &self.cfdtmdf8_32_0
    }
    ///0x11030 - TX Message Buffer Data Field 9 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_32_0(&self) -> &Cfdtmdf9_0 {
        &self.cfdtmdf9_32_0
    }
    ///0x11034 - TX Message Buffer Data Field 10 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_32_0(&self) -> &Cfdtmdf10_0 {
        &self.cfdtmdf10_32_0
    }
    ///0x11038 - TX Message Buffer Data Field 11 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_32_0(&self) -> &Cfdtmdf11_0 {
        &self.cfdtmdf11_32_0
    }
    ///0x1103c - TX Message Buffer Data Field 12 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_32_0(&self) -> &Cfdtmdf12_0 {
        &self.cfdtmdf12_32_0
    }
    ///0x11040 - TX Message Buffer Data Field 13 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_32_0(&self) -> &Cfdtmdf13_0 {
        &self.cfdtmdf13_32_0
    }
    ///0x11044 - TX Message Buffer Data Field 14 Register 32 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_32_0(&self) -> &Cfdtmdf14_0 {
        &self.cfdtmdf14_32_0
    }
    ///0x11048 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_32_0(&self) -> &Cfdtmdf15_0 {
        &self.cfdtmdf15_32_0
    }
    ///0x11000 - TX Message Buffer ID Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmid33_0(&self) -> &Cfdtmid0 {
        &self.cfdtmid33_0
    }
    ///0x11004 - TX Message Buffer Pointer Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr33_0(&self) -> &Cfdtmptr0 {
        &self.cfdtmptr33_0
    }
    ///0x11008 - TX Message Buffer CANFD Control Register 33 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr33_0(&self) -> &Cfdtmfdctr0 {
        &self.cfdtmfdctr33_0
    }
    ///0x1100c - TX Message Buffer Data Field 0 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_33_0(&self) -> &Cfdtmdf0_0 {
        &self.cfdtmdf0_33_0
    }
    ///0x11010 - TX Message Buffer Data Field 1 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_33_0(&self) -> &Cfdtmdf1_0 {
        &self.cfdtmdf1_33_0
    }
    ///0x11014 - TX Message Buffer Data Field 2 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_33_0(&self) -> &Cfdtmdf2_0 {
        &self.cfdtmdf2_33_0
    }
    ///0x11018 - TX Message Buffer Data Field 3 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_33_0(&self) -> &Cfdtmdf3_0 {
        &self.cfdtmdf3_33_0
    }
    ///0x1101c - TX Message Buffer Data Field 4 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_33_0(&self) -> &Cfdtmdf4_0 {
        &self.cfdtmdf4_33_0
    }
    ///0x11020 - TX Message Buffer Data Field 5 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_33_0(&self) -> &Cfdtmdf5_0 {
        &self.cfdtmdf5_33_0
    }
    ///0x11024 - TX Message Buffer Data Field 6 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_33_0(&self) -> &Cfdtmdf6_0 {
        &self.cfdtmdf6_33_0
    }
    ///0x11028 - TX Message Buffer Data Field 7 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_33_0(&self) -> &Cfdtmdf7_0 {
        &self.cfdtmdf7_33_0
    }
    ///0x1102c - TX Message Buffer Data Field 8 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_33_0(&self) -> &Cfdtmdf8_0 {
        &self.cfdtmdf8_33_0
    }
    ///0x11030 - TX Message Buffer Data Field 9 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_33_0(&self) -> &Cfdtmdf9_0 {
        &self.cfdtmdf9_33_0
    }
    ///0x11034 - TX Message Buffer Data Field 10 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_33_0(&self) -> &Cfdtmdf10_0 {
        &self.cfdtmdf10_33_0
    }
    ///0x11038 - TX Message Buffer Data Field 11 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_33_0(&self) -> &Cfdtmdf11_0 {
        &self.cfdtmdf11_33_0
    }
    ///0x1103c - TX Message Buffer Data Field 12 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_33_0(&self) -> &Cfdtmdf12_0 {
        &self.cfdtmdf12_33_0
    }
    ///0x11040 - TX Message Buffer Data Field 13 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_33_0(&self) -> &Cfdtmdf13_0 {
        &self.cfdtmdf13_33_0
    }
    ///0x11044 - TX Message Buffer Data Field 14 Register 33 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_33_0(&self) -> &Cfdtmdf14_0 {
        &self.cfdtmdf14_33_0
    }
    ///0x11048 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_33_0(&self) -> &Cfdtmdf15_0 {
        &self.cfdtmdf15_33_0
    }
    ///0x11000 - TX Message Buffer ID Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmid34_0(&self) -> &Cfdtmid0 {
        &self.cfdtmid34_0
    }
    ///0x11004 - TX Message Buffer Pointer Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr34_0(&self) -> &Cfdtmptr0 {
        &self.cfdtmptr34_0
    }
    ///0x11008 - TX Message Buffer CANFD Control Register 34 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr34_0(&self) -> &Cfdtmfdctr0 {
        &self.cfdtmfdctr34_0
    }
    ///0x1100c - TX Message Buffer Data Field 0 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_34_0(&self) -> &Cfdtmdf0_0 {
        &self.cfdtmdf0_34_0
    }
    ///0x11010 - TX Message Buffer Data Field 1 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_34_0(&self) -> &Cfdtmdf1_0 {
        &self.cfdtmdf1_34_0
    }
    ///0x11014 - TX Message Buffer Data Field 2 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_34_0(&self) -> &Cfdtmdf2_0 {
        &self.cfdtmdf2_34_0
    }
    ///0x11018 - TX Message Buffer Data Field 3 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_34_0(&self) -> &Cfdtmdf3_0 {
        &self.cfdtmdf3_34_0
    }
    ///0x1101c - TX Message Buffer Data Field 4 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_34_0(&self) -> &Cfdtmdf4_0 {
        &self.cfdtmdf4_34_0
    }
    ///0x11020 - TX Message Buffer Data Field 5 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_34_0(&self) -> &Cfdtmdf5_0 {
        &self.cfdtmdf5_34_0
    }
    ///0x11024 - TX Message Buffer Data Field 6 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_34_0(&self) -> &Cfdtmdf6_0 {
        &self.cfdtmdf6_34_0
    }
    ///0x11028 - TX Message Buffer Data Field 7 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_34_0(&self) -> &Cfdtmdf7_0 {
        &self.cfdtmdf7_34_0
    }
    ///0x1102c - TX Message Buffer Data Field 8 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_34_0(&self) -> &Cfdtmdf8_0 {
        &self.cfdtmdf8_34_0
    }
    ///0x11030 - TX Message Buffer Data Field 9 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_34_0(&self) -> &Cfdtmdf9_0 {
        &self.cfdtmdf9_34_0
    }
    ///0x11034 - TX Message Buffer Data Field 10 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_34_0(&self) -> &Cfdtmdf10_0 {
        &self.cfdtmdf10_34_0
    }
    ///0x11038 - TX Message Buffer Data Field 11 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_34_0(&self) -> &Cfdtmdf11_0 {
        &self.cfdtmdf11_34_0
    }
    ///0x1103c - TX Message Buffer Data Field 12 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_34_0(&self) -> &Cfdtmdf12_0 {
        &self.cfdtmdf12_34_0
    }
    ///0x11040 - TX Message Buffer Data Field 13 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_34_0(&self) -> &Cfdtmdf13_0 {
        &self.cfdtmdf13_34_0
    }
    ///0x11044 - TX Message Buffer Data Field 14 Register 34 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_34_0(&self) -> &Cfdtmdf14_0 {
        &self.cfdtmdf14_34_0
    }
    ///0x11048 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_34_0(&self) -> &Cfdtmdf15_0 {
        &self.cfdtmdf15_34_0
    }
    ///0x11000 - TX Message Buffer ID Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmid35_0(&self) -> &Cfdtmid0 {
        &self.cfdtmid35_0
    }
    ///0x11004 - TX Message Buffer Pointer Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr35_0(&self) -> &Cfdtmptr0 {
        &self.cfdtmptr35_0
    }
    ///0x11008 - TX Message Buffer CANFD Control Register 35 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr35_0(&self) -> &Cfdtmfdctr0 {
        &self.cfdtmfdctr35_0
    }
    ///0x1100c - TX Message Buffer Data Field 0 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_35_0(&self) -> &Cfdtmdf0_0 {
        &self.cfdtmdf0_35_0
    }
    ///0x11010 - TX Message Buffer Data Field 1 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_35_0(&self) -> &Cfdtmdf1_0 {
        &self.cfdtmdf1_35_0
    }
    ///0x11014 - TX Message Buffer Data Field 2 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_35_0(&self) -> &Cfdtmdf2_0 {
        &self.cfdtmdf2_35_0
    }
    ///0x11018 - TX Message Buffer Data Field 3 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_35_0(&self) -> &Cfdtmdf3_0 {
        &self.cfdtmdf3_35_0
    }
    ///0x1101c - TX Message Buffer Data Field 4 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_35_0(&self) -> &Cfdtmdf4_0 {
        &self.cfdtmdf4_35_0
    }
    ///0x11020 - TX Message Buffer Data Field 5 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_35_0(&self) -> &Cfdtmdf5_0 {
        &self.cfdtmdf5_35_0
    }
    ///0x11024 - TX Message Buffer Data Field 6 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_35_0(&self) -> &Cfdtmdf6_0 {
        &self.cfdtmdf6_35_0
    }
    ///0x11028 - TX Message Buffer Data Field 7 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_35_0(&self) -> &Cfdtmdf7_0 {
        &self.cfdtmdf7_35_0
    }
    ///0x1102c - TX Message Buffer Data Field 8 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_35_0(&self) -> &Cfdtmdf8_0 {
        &self.cfdtmdf8_35_0
    }
    ///0x11030 - TX Message Buffer Data Field 9 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_35_0(&self) -> &Cfdtmdf9_0 {
        &self.cfdtmdf9_35_0
    }
    ///0x11034 - TX Message Buffer Data Field 10 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_35_0(&self) -> &Cfdtmdf10_0 {
        &self.cfdtmdf10_35_0
    }
    ///0x11038 - TX Message Buffer Data Field 11 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_35_0(&self) -> &Cfdtmdf11_0 {
        &self.cfdtmdf11_35_0
    }
    ///0x1103c - TX Message Buffer Data Field 12 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_35_0(&self) -> &Cfdtmdf12_0 {
        &self.cfdtmdf12_35_0
    }
    ///0x11040 - TX Message Buffer Data Field 13 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_35_0(&self) -> &Cfdtmdf13_0 {
        &self.cfdtmdf13_35_0
    }
    ///0x11044 - TX Message Buffer Data Field 14 Register 35 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_35_0(&self) -> &Cfdtmdf14_0 {
        &self.cfdtmdf14_35_0
    }
    ///0x11048 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_35_0(&self) -> &Cfdtmdf15_0 {
        &self.cfdtmdf15_35_0
    }
    ///0x11000 - TX Message Buffer ID Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmid36_0(&self) -> &Cfdtmid0 {
        &self.cfdtmid36_0
    }
    ///0x11004 - TX Message Buffer Pointer Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr36_0(&self) -> &Cfdtmptr0 {
        &self.cfdtmptr36_0
    }
    ///0x11008 - TX Message Buffer CANFD Control Register 36 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr36_0(&self) -> &Cfdtmfdctr0 {
        &self.cfdtmfdctr36_0
    }
    ///0x1100c - TX Message Buffer Data Field 0 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_36_0(&self) -> &Cfdtmdf0_0 {
        &self.cfdtmdf0_36_0
    }
    ///0x11010 - TX Message Buffer Data Field 1 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_36_0(&self) -> &Cfdtmdf1_0 {
        &self.cfdtmdf1_36_0
    }
    ///0x11014 - TX Message Buffer Data Field 2 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_36_0(&self) -> &Cfdtmdf2_0 {
        &self.cfdtmdf2_36_0
    }
    ///0x11018 - TX Message Buffer Data Field 3 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_36_0(&self) -> &Cfdtmdf3_0 {
        &self.cfdtmdf3_36_0
    }
    ///0x1101c - TX Message Buffer Data Field 4 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_36_0(&self) -> &Cfdtmdf4_0 {
        &self.cfdtmdf4_36_0
    }
    ///0x11020 - TX Message Buffer Data Field 5 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_36_0(&self) -> &Cfdtmdf5_0 {
        &self.cfdtmdf5_36_0
    }
    ///0x11024 - TX Message Buffer Data Field 6 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_36_0(&self) -> &Cfdtmdf6_0 {
        &self.cfdtmdf6_36_0
    }
    ///0x11028 - TX Message Buffer Data Field 7 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_36_0(&self) -> &Cfdtmdf7_0 {
        &self.cfdtmdf7_36_0
    }
    ///0x1102c - TX Message Buffer Data Field 8 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_36_0(&self) -> &Cfdtmdf8_0 {
        &self.cfdtmdf8_36_0
    }
    ///0x11030 - TX Message Buffer Data Field 9 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_36_0(&self) -> &Cfdtmdf9_0 {
        &self.cfdtmdf9_36_0
    }
    ///0x11034 - TX Message Buffer Data Field 10 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_36_0(&self) -> &Cfdtmdf10_0 {
        &self.cfdtmdf10_36_0
    }
    ///0x11038 - TX Message Buffer Data Field 11 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_36_0(&self) -> &Cfdtmdf11_0 {
        &self.cfdtmdf11_36_0
    }
    ///0x1103c - TX Message Buffer Data Field 12 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_36_0(&self) -> &Cfdtmdf12_0 {
        &self.cfdtmdf12_36_0
    }
    ///0x11040 - TX Message Buffer Data Field 13 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_36_0(&self) -> &Cfdtmdf13_0 {
        &self.cfdtmdf13_36_0
    }
    ///0x11044 - TX Message Buffer Data Field 14 Register 36 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_36_0(&self) -> &Cfdtmdf14_0 {
        &self.cfdtmdf14_36_0
    }
    ///0x11048 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_36_0(&self) -> &Cfdtmdf15_0 {
        &self.cfdtmdf15_36_0
    }
    ///0x11000 - TX Message Buffer ID Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmid37_0(&self) -> &Cfdtmid0 {
        &self.cfdtmid37_0
    }
    ///0x11004 - TX Message Buffer Pointer Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr37_0(&self) -> &Cfdtmptr0 {
        &self.cfdtmptr37_0
    }
    ///0x11008 - TX Message Buffer CANFD Control Register 37 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr37_0(&self) -> &Cfdtmfdctr0 {
        &self.cfdtmfdctr37_0
    }
    ///0x1100c - TX Message Buffer Data Field 0 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_37_0(&self) -> &Cfdtmdf0_0 {
        &self.cfdtmdf0_37_0
    }
    ///0x11010 - TX Message Buffer Data Field 1 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_37_0(&self) -> &Cfdtmdf1_0 {
        &self.cfdtmdf1_37_0
    }
    ///0x11014 - TX Message Buffer Data Field 2 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_37_0(&self) -> &Cfdtmdf2_0 {
        &self.cfdtmdf2_37_0
    }
    ///0x11018 - TX Message Buffer Data Field 3 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_37_0(&self) -> &Cfdtmdf3_0 {
        &self.cfdtmdf3_37_0
    }
    ///0x1101c - TX Message Buffer Data Field 4 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_37_0(&self) -> &Cfdtmdf4_0 {
        &self.cfdtmdf4_37_0
    }
    ///0x11020 - TX Message Buffer Data Field 5 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_37_0(&self) -> &Cfdtmdf5_0 {
        &self.cfdtmdf5_37_0
    }
    ///0x11024 - TX Message Buffer Data Field 6 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_37_0(&self) -> &Cfdtmdf6_0 {
        &self.cfdtmdf6_37_0
    }
    ///0x11028 - TX Message Buffer Data Field 7 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_37_0(&self) -> &Cfdtmdf7_0 {
        &self.cfdtmdf7_37_0
    }
    ///0x1102c - TX Message Buffer Data Field 8 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_37_0(&self) -> &Cfdtmdf8_0 {
        &self.cfdtmdf8_37_0
    }
    ///0x11030 - TX Message Buffer Data Field 9 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_37_0(&self) -> &Cfdtmdf9_0 {
        &self.cfdtmdf9_37_0
    }
    ///0x11034 - TX Message Buffer Data Field 10 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_37_0(&self) -> &Cfdtmdf10_0 {
        &self.cfdtmdf10_37_0
    }
    ///0x11038 - TX Message Buffer Data Field 11 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_37_0(&self) -> &Cfdtmdf11_0 {
        &self.cfdtmdf11_37_0
    }
    ///0x1103c - TX Message Buffer Data Field 12 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_37_0(&self) -> &Cfdtmdf12_0 {
        &self.cfdtmdf12_37_0
    }
    ///0x11040 - TX Message Buffer Data Field 13 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_37_0(&self) -> &Cfdtmdf13_0 {
        &self.cfdtmdf13_37_0
    }
    ///0x11044 - TX Message Buffer Data Field 14 Register 37 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_37_0(&self) -> &Cfdtmdf14_0 {
        &self.cfdtmdf14_37_0
    }
    ///0x11048 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_37_0(&self) -> &Cfdtmdf15_0 {
        &self.cfdtmdf15_37_0
    }
    ///0x11000 - TX Message Buffer ID Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmid38_0(&self) -> &Cfdtmid0 {
        &self.cfdtmid38_0
    }
    ///0x11004 - TX Message Buffer Pointer Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr38_0(&self) -> &Cfdtmptr0 {
        &self.cfdtmptr38_0
    }
    ///0x11008 - TX Message Buffer CANFD Control Register 38 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr38_0(&self) -> &Cfdtmfdctr0 {
        &self.cfdtmfdctr38_0
    }
    ///0x1100c - TX Message Buffer Data Field 0 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_38_0(&self) -> &Cfdtmdf0_0 {
        &self.cfdtmdf0_38_0
    }
    ///0x11010 - TX Message Buffer Data Field 1 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_38_0(&self) -> &Cfdtmdf1_0 {
        &self.cfdtmdf1_38_0
    }
    ///0x11014 - TX Message Buffer Data Field 2 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_38_0(&self) -> &Cfdtmdf2_0 {
        &self.cfdtmdf2_38_0
    }
    ///0x11018 - TX Message Buffer Data Field 3 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_38_0(&self) -> &Cfdtmdf3_0 {
        &self.cfdtmdf3_38_0
    }
    ///0x1101c - TX Message Buffer Data Field 4 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_38_0(&self) -> &Cfdtmdf4_0 {
        &self.cfdtmdf4_38_0
    }
    ///0x11020 - TX Message Buffer Data Field 5 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_38_0(&self) -> &Cfdtmdf5_0 {
        &self.cfdtmdf5_38_0
    }
    ///0x11024 - TX Message Buffer Data Field 6 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_38_0(&self) -> &Cfdtmdf6_0 {
        &self.cfdtmdf6_38_0
    }
    ///0x11028 - TX Message Buffer Data Field 7 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_38_0(&self) -> &Cfdtmdf7_0 {
        &self.cfdtmdf7_38_0
    }
    ///0x1102c - TX Message Buffer Data Field 8 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_38_0(&self) -> &Cfdtmdf8_0 {
        &self.cfdtmdf8_38_0
    }
    ///0x11030 - TX Message Buffer Data Field 9 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_38_0(&self) -> &Cfdtmdf9_0 {
        &self.cfdtmdf9_38_0
    }
    ///0x11034 - TX Message Buffer Data Field 10 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_38_0(&self) -> &Cfdtmdf10_0 {
        &self.cfdtmdf10_38_0
    }
    ///0x11038 - TX Message Buffer Data Field 11 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_38_0(&self) -> &Cfdtmdf11_0 {
        &self.cfdtmdf11_38_0
    }
    ///0x1103c - TX Message Buffer Data Field 12 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_38_0(&self) -> &Cfdtmdf12_0 {
        &self.cfdtmdf12_38_0
    }
    ///0x11040 - TX Message Buffer Data Field 13 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_38_0(&self) -> &Cfdtmdf13_0 {
        &self.cfdtmdf13_38_0
    }
    ///0x11044 - TX Message Buffer Data Field 14 Register 38 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_38_0(&self) -> &Cfdtmdf14_0 {
        &self.cfdtmdf14_38_0
    }
    ///0x11048 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_38_0(&self) -> &Cfdtmdf15_0 {
        &self.cfdtmdf15_38_0
    }
    ///0x11000 - TX Message Buffer ID Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmid39_0(&self) -> &Cfdtmid0 {
        &self.cfdtmid39_0
    }
    ///0x11004 - TX Message Buffer Pointer Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmptr39_0(&self) -> &Cfdtmptr0 {
        &self.cfdtmptr39_0
    }
    ///0x11008 - TX Message Buffer CANFD Control Register 39 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr39_0(&self) -> &Cfdtmfdctr0 {
        &self.cfdtmfdctr39_0
    }
    ///0x1100c - TX Message Buffer Data Field 0 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf0_39_0(&self) -> &Cfdtmdf0_0 {
        &self.cfdtmdf0_39_0
    }
    ///0x11010 - TX Message Buffer Data Field 1 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf1_39_0(&self) -> &Cfdtmdf1_0 {
        &self.cfdtmdf1_39_0
    }
    ///0x11014 - TX Message Buffer Data Field 2 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf2_39_0(&self) -> &Cfdtmdf2_0 {
        &self.cfdtmdf2_39_0
    }
    ///0x11018 - TX Message Buffer Data Field 3 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf3_39_0(&self) -> &Cfdtmdf3_0 {
        &self.cfdtmdf3_39_0
    }
    ///0x1101c - TX Message Buffer Data Field 4 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf4_39_0(&self) -> &Cfdtmdf4_0 {
        &self.cfdtmdf4_39_0
    }
    ///0x11020 - TX Message Buffer Data Field 5 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf5_39_0(&self) -> &Cfdtmdf5_0 {
        &self.cfdtmdf5_39_0
    }
    ///0x11024 - TX Message Buffer Data Field 6 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf6_39_0(&self) -> &Cfdtmdf6_0 {
        &self.cfdtmdf6_39_0
    }
    ///0x11028 - TX Message Buffer Data Field 7 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf7_39_0(&self) -> &Cfdtmdf7_0 {
        &self.cfdtmdf7_39_0
    }
    ///0x1102c - TX Message Buffer Data Field 8 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf8_39_0(&self) -> &Cfdtmdf8_0 {
        &self.cfdtmdf8_39_0
    }
    ///0x11030 - TX Message Buffer Data Field 9 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf9_39_0(&self) -> &Cfdtmdf9_0 {
        &self.cfdtmdf9_39_0
    }
    ///0x11034 - TX Message Buffer Data Field 10 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf10_39_0(&self) -> &Cfdtmdf10_0 {
        &self.cfdtmdf10_39_0
    }
    ///0x11038 - TX Message Buffer Data Field 11 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf11_39_0(&self) -> &Cfdtmdf11_0 {
        &self.cfdtmdf11_39_0
    }
    ///0x1103c - TX Message Buffer Data Field 12 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf12_39_0(&self) -> &Cfdtmdf12_0 {
        &self.cfdtmdf12_39_0
    }
    ///0x11040 - TX Message Buffer Data Field 13 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf13_39_0(&self) -> &Cfdtmdf13_0 {
        &self.cfdtmdf13_39_0
    }
    ///0x11044 - TX Message Buffer Data Field 14 Register 39 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf14_39_0(&self) -> &Cfdtmdf14_0 {
        &self.cfdtmdf14_39_0
    }
    ///0x11048 - TX Message Buffer Data Field X Register 15 Channel 0
    #[inline(always)]
    pub const fn cfdtmdf15_39_0(&self) -> &Cfdtmdf15_0 {
        &self.cfdtmdf15_39_0
    }
    ///0x12000..0x12020 - TX Message Buffer ID Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmid_1(&self, n: usize) -> &Cfdtmid1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73728).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12000..0x12020 - TX Message Buffer ID Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmid_1_iter(&self) -> impl Iterator<Item = &Cfdtmid1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73728).add(128 * n).cast()
            })
    }
    ///0x12000 - TX Message Buffer ID Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmid0_1(&self) -> &Cfdtmid1 {
        self.cfdtmid_1(0)
    }
    ///0x12080 - TX Message Buffer ID Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmid1_1(&self) -> &Cfdtmid1 {
        self.cfdtmid_1(1)
    }
    ///0x12100 - TX Message Buffer ID Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmid2_1(&self) -> &Cfdtmid1 {
        self.cfdtmid_1(2)
    }
    ///0x12180 - TX Message Buffer ID Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmid3_1(&self) -> &Cfdtmid1 {
        self.cfdtmid_1(3)
    }
    ///0x12200 - TX Message Buffer ID Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmid4_1(&self) -> &Cfdtmid1 {
        self.cfdtmid_1(4)
    }
    ///0x12280 - TX Message Buffer ID Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmid5_1(&self) -> &Cfdtmid1 {
        self.cfdtmid_1(5)
    }
    ///0x12300 - TX Message Buffer ID Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmid6_1(&self) -> &Cfdtmid1 {
        self.cfdtmid_1(6)
    }
    ///0x12380 - TX Message Buffer ID Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmid7_1(&self) -> &Cfdtmid1 {
        self.cfdtmid_1(7)
    }
    ///0x12004..0x12024 - TX Message Buffer Pointer Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmptr_1(&self, n: usize) -> &Cfdtmptr1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73732).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12004..0x12024 - TX Message Buffer Pointer Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmptr_1_iter(&self) -> impl Iterator<Item = &Cfdtmptr1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73732).add(128 * n).cast()
            })
    }
    ///0x12004 - TX Message Buffer Pointer Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr0_1(&self) -> &Cfdtmptr1 {
        self.cfdtmptr_1(0)
    }
    ///0x12084 - TX Message Buffer Pointer Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr1_1(&self) -> &Cfdtmptr1 {
        self.cfdtmptr_1(1)
    }
    ///0x12104 - TX Message Buffer Pointer Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr2_1(&self) -> &Cfdtmptr1 {
        self.cfdtmptr_1(2)
    }
    ///0x12184 - TX Message Buffer Pointer Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr3_1(&self) -> &Cfdtmptr1 {
        self.cfdtmptr_1(3)
    }
    ///0x12204 - TX Message Buffer Pointer Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr4_1(&self) -> &Cfdtmptr1 {
        self.cfdtmptr_1(4)
    }
    ///0x12284 - TX Message Buffer Pointer Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr5_1(&self) -> &Cfdtmptr1 {
        self.cfdtmptr_1(5)
    }
    ///0x12304 - TX Message Buffer Pointer Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr6_1(&self) -> &Cfdtmptr1 {
        self.cfdtmptr_1(6)
    }
    ///0x12384 - TX Message Buffer Pointer Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr7_1(&self) -> &Cfdtmptr1 {
        self.cfdtmptr_1(7)
    }
    ///0x12008..0x12028 - TX Message Buffer CANFD Control Register %s Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr_1(&self, n: usize) -> &Cfdtmfdctr1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73736).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12008..0x12028 - TX Message Buffer CANFD Control Register %s Channel i
    #[inline(always)]
    pub fn cfdtmfdctr_1_iter(&self) -> impl Iterator<Item = &Cfdtmfdctr1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73736).add(128 * n).cast()
            })
    }
    ///0x12008 - TX Message Buffer CANFD Control Register 0 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr0_1(&self) -> &Cfdtmfdctr1 {
        self.cfdtmfdctr_1(0)
    }
    ///0x12088 - TX Message Buffer CANFD Control Register 1 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr1_1(&self) -> &Cfdtmfdctr1 {
        self.cfdtmfdctr_1(1)
    }
    ///0x12108 - TX Message Buffer CANFD Control Register 2 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr2_1(&self) -> &Cfdtmfdctr1 {
        self.cfdtmfdctr_1(2)
    }
    ///0x12188 - TX Message Buffer CANFD Control Register 3 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr3_1(&self) -> &Cfdtmfdctr1 {
        self.cfdtmfdctr_1(3)
    }
    ///0x12208 - TX Message Buffer CANFD Control Register 4 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr4_1(&self) -> &Cfdtmfdctr1 {
        self.cfdtmfdctr_1(4)
    }
    ///0x12288 - TX Message Buffer CANFD Control Register 5 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr5_1(&self) -> &Cfdtmfdctr1 {
        self.cfdtmfdctr_1(5)
    }
    ///0x12308 - TX Message Buffer CANFD Control Register 6 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr6_1(&self) -> &Cfdtmfdctr1 {
        self.cfdtmfdctr_1(6)
    }
    ///0x12388 - TX Message Buffer CANFD Control Register 7 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr7_1(&self) -> &Cfdtmfdctr1 {
        self.cfdtmfdctr_1(7)
    }
    ///0x1200c..0x1202c - TX Message Buffer Data Field 0 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0__1(&self, n: usize) -> &Cfdtmdf0_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73740).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x1200c..0x1202c - TX Message Buffer Data Field 0 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf0__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf0_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73740).add(128 * n).cast()
            })
    }
    ///0x1200c - TX Message Buffer Data Field 0 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_0_1(&self) -> &Cfdtmdf0_1 {
        self.cfdtmdf0__1(0)
    }
    ///0x1208c - TX Message Buffer Data Field 0 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_1_1(&self) -> &Cfdtmdf0_1 {
        self.cfdtmdf0__1(1)
    }
    ///0x1210c - TX Message Buffer Data Field 0 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_2_1(&self) -> &Cfdtmdf0_1 {
        self.cfdtmdf0__1(2)
    }
    ///0x1218c - TX Message Buffer Data Field 0 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_3_1(&self) -> &Cfdtmdf0_1 {
        self.cfdtmdf0__1(3)
    }
    ///0x1220c - TX Message Buffer Data Field 0 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_4_1(&self) -> &Cfdtmdf0_1 {
        self.cfdtmdf0__1(4)
    }
    ///0x1228c - TX Message Buffer Data Field 0 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_5_1(&self) -> &Cfdtmdf0_1 {
        self.cfdtmdf0__1(5)
    }
    ///0x1230c - TX Message Buffer Data Field 0 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_6_1(&self) -> &Cfdtmdf0_1 {
        self.cfdtmdf0__1(6)
    }
    ///0x1238c - TX Message Buffer Data Field 0 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_7_1(&self) -> &Cfdtmdf0_1 {
        self.cfdtmdf0__1(7)
    }
    ///0x12010..0x12030 - TX Message Buffer Data Field 1 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1__1(&self, n: usize) -> &Cfdtmdf1_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73744).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12010..0x12030 - TX Message Buffer Data Field 1 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf1__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf1_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73744).add(128 * n).cast()
            })
    }
    ///0x12010 - TX Message Buffer Data Field 1 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_0_1(&self) -> &Cfdtmdf1_1 {
        self.cfdtmdf1__1(0)
    }
    ///0x12090 - TX Message Buffer Data Field 1 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_1_1(&self) -> &Cfdtmdf1_1 {
        self.cfdtmdf1__1(1)
    }
    ///0x12110 - TX Message Buffer Data Field 1 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_2_1(&self) -> &Cfdtmdf1_1 {
        self.cfdtmdf1__1(2)
    }
    ///0x12190 - TX Message Buffer Data Field 1 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_3_1(&self) -> &Cfdtmdf1_1 {
        self.cfdtmdf1__1(3)
    }
    ///0x12210 - TX Message Buffer Data Field 1 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_4_1(&self) -> &Cfdtmdf1_1 {
        self.cfdtmdf1__1(4)
    }
    ///0x12290 - TX Message Buffer Data Field 1 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_5_1(&self) -> &Cfdtmdf1_1 {
        self.cfdtmdf1__1(5)
    }
    ///0x12310 - TX Message Buffer Data Field 1 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_6_1(&self) -> &Cfdtmdf1_1 {
        self.cfdtmdf1__1(6)
    }
    ///0x12390 - TX Message Buffer Data Field 1 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_7_1(&self) -> &Cfdtmdf1_1 {
        self.cfdtmdf1__1(7)
    }
    ///0x12014..0x12034 - TX Message Buffer Data Field 2 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2__1(&self, n: usize) -> &Cfdtmdf2_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73748).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12014..0x12034 - TX Message Buffer Data Field 2 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf2__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf2_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73748).add(128 * n).cast()
            })
    }
    ///0x12014 - TX Message Buffer Data Field 2 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_0_1(&self) -> &Cfdtmdf2_1 {
        self.cfdtmdf2__1(0)
    }
    ///0x12094 - TX Message Buffer Data Field 2 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_1_1(&self) -> &Cfdtmdf2_1 {
        self.cfdtmdf2__1(1)
    }
    ///0x12114 - TX Message Buffer Data Field 2 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_2_1(&self) -> &Cfdtmdf2_1 {
        self.cfdtmdf2__1(2)
    }
    ///0x12194 - TX Message Buffer Data Field 2 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_3_1(&self) -> &Cfdtmdf2_1 {
        self.cfdtmdf2__1(3)
    }
    ///0x12214 - TX Message Buffer Data Field 2 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_4_1(&self) -> &Cfdtmdf2_1 {
        self.cfdtmdf2__1(4)
    }
    ///0x12294 - TX Message Buffer Data Field 2 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_5_1(&self) -> &Cfdtmdf2_1 {
        self.cfdtmdf2__1(5)
    }
    ///0x12314 - TX Message Buffer Data Field 2 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_6_1(&self) -> &Cfdtmdf2_1 {
        self.cfdtmdf2__1(6)
    }
    ///0x12394 - TX Message Buffer Data Field 2 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_7_1(&self) -> &Cfdtmdf2_1 {
        self.cfdtmdf2__1(7)
    }
    ///0x12018..0x12038 - TX Message Buffer Data Field 3 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3__1(&self, n: usize) -> &Cfdtmdf3_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73752).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12018..0x12038 - TX Message Buffer Data Field 3 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf3__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf3_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73752).add(128 * n).cast()
            })
    }
    ///0x12018 - TX Message Buffer Data Field 3 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_0_1(&self) -> &Cfdtmdf3_1 {
        self.cfdtmdf3__1(0)
    }
    ///0x12098 - TX Message Buffer Data Field 3 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_1_1(&self) -> &Cfdtmdf3_1 {
        self.cfdtmdf3__1(1)
    }
    ///0x12118 - TX Message Buffer Data Field 3 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_2_1(&self) -> &Cfdtmdf3_1 {
        self.cfdtmdf3__1(2)
    }
    ///0x12198 - TX Message Buffer Data Field 3 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_3_1(&self) -> &Cfdtmdf3_1 {
        self.cfdtmdf3__1(3)
    }
    ///0x12218 - TX Message Buffer Data Field 3 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_4_1(&self) -> &Cfdtmdf3_1 {
        self.cfdtmdf3__1(4)
    }
    ///0x12298 - TX Message Buffer Data Field 3 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_5_1(&self) -> &Cfdtmdf3_1 {
        self.cfdtmdf3__1(5)
    }
    ///0x12318 - TX Message Buffer Data Field 3 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_6_1(&self) -> &Cfdtmdf3_1 {
        self.cfdtmdf3__1(6)
    }
    ///0x12398 - TX Message Buffer Data Field 3 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_7_1(&self) -> &Cfdtmdf3_1 {
        self.cfdtmdf3__1(7)
    }
    ///0x1201c..0x1203c - TX Message Buffer Data Field 4 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4__1(&self, n: usize) -> &Cfdtmdf4_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73756).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x1201c..0x1203c - TX Message Buffer Data Field 4 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf4__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf4_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73756).add(128 * n).cast()
            })
    }
    ///0x1201c - TX Message Buffer Data Field 4 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_0_1(&self) -> &Cfdtmdf4_1 {
        self.cfdtmdf4__1(0)
    }
    ///0x1209c - TX Message Buffer Data Field 4 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_1_1(&self) -> &Cfdtmdf4_1 {
        self.cfdtmdf4__1(1)
    }
    ///0x1211c - TX Message Buffer Data Field 4 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_2_1(&self) -> &Cfdtmdf4_1 {
        self.cfdtmdf4__1(2)
    }
    ///0x1219c - TX Message Buffer Data Field 4 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_3_1(&self) -> &Cfdtmdf4_1 {
        self.cfdtmdf4__1(3)
    }
    ///0x1221c - TX Message Buffer Data Field 4 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_4_1(&self) -> &Cfdtmdf4_1 {
        self.cfdtmdf4__1(4)
    }
    ///0x1229c - TX Message Buffer Data Field 4 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_5_1(&self) -> &Cfdtmdf4_1 {
        self.cfdtmdf4__1(5)
    }
    ///0x1231c - TX Message Buffer Data Field 4 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_6_1(&self) -> &Cfdtmdf4_1 {
        self.cfdtmdf4__1(6)
    }
    ///0x1239c - TX Message Buffer Data Field 4 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_7_1(&self) -> &Cfdtmdf4_1 {
        self.cfdtmdf4__1(7)
    }
    ///0x12020..0x12040 - TX Message Buffer Data Field 5 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5__1(&self, n: usize) -> &Cfdtmdf5_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73760).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12020..0x12040 - TX Message Buffer Data Field 5 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf5__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf5_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73760).add(128 * n).cast()
            })
    }
    ///0x12020 - TX Message Buffer Data Field 5 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_0_1(&self) -> &Cfdtmdf5_1 {
        self.cfdtmdf5__1(0)
    }
    ///0x120a0 - TX Message Buffer Data Field 5 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_1_1(&self) -> &Cfdtmdf5_1 {
        self.cfdtmdf5__1(1)
    }
    ///0x12120 - TX Message Buffer Data Field 5 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_2_1(&self) -> &Cfdtmdf5_1 {
        self.cfdtmdf5__1(2)
    }
    ///0x121a0 - TX Message Buffer Data Field 5 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_3_1(&self) -> &Cfdtmdf5_1 {
        self.cfdtmdf5__1(3)
    }
    ///0x12220 - TX Message Buffer Data Field 5 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_4_1(&self) -> &Cfdtmdf5_1 {
        self.cfdtmdf5__1(4)
    }
    ///0x122a0 - TX Message Buffer Data Field 5 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_5_1(&self) -> &Cfdtmdf5_1 {
        self.cfdtmdf5__1(5)
    }
    ///0x12320 - TX Message Buffer Data Field 5 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_6_1(&self) -> &Cfdtmdf5_1 {
        self.cfdtmdf5__1(6)
    }
    ///0x123a0 - TX Message Buffer Data Field 5 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_7_1(&self) -> &Cfdtmdf5_1 {
        self.cfdtmdf5__1(7)
    }
    ///0x12024..0x12044 - TX Message Buffer Data Field 6 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6__1(&self, n: usize) -> &Cfdtmdf6_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73764).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12024..0x12044 - TX Message Buffer Data Field 6 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf6__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf6_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73764).add(128 * n).cast()
            })
    }
    ///0x12024 - TX Message Buffer Data Field 6 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_0_1(&self) -> &Cfdtmdf6_1 {
        self.cfdtmdf6__1(0)
    }
    ///0x120a4 - TX Message Buffer Data Field 6 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_1_1(&self) -> &Cfdtmdf6_1 {
        self.cfdtmdf6__1(1)
    }
    ///0x12124 - TX Message Buffer Data Field 6 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_2_1(&self) -> &Cfdtmdf6_1 {
        self.cfdtmdf6__1(2)
    }
    ///0x121a4 - TX Message Buffer Data Field 6 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_3_1(&self) -> &Cfdtmdf6_1 {
        self.cfdtmdf6__1(3)
    }
    ///0x12224 - TX Message Buffer Data Field 6 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_4_1(&self) -> &Cfdtmdf6_1 {
        self.cfdtmdf6__1(4)
    }
    ///0x122a4 - TX Message Buffer Data Field 6 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_5_1(&self) -> &Cfdtmdf6_1 {
        self.cfdtmdf6__1(5)
    }
    ///0x12324 - TX Message Buffer Data Field 6 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_6_1(&self) -> &Cfdtmdf6_1 {
        self.cfdtmdf6__1(6)
    }
    ///0x123a4 - TX Message Buffer Data Field 6 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_7_1(&self) -> &Cfdtmdf6_1 {
        self.cfdtmdf6__1(7)
    }
    ///0x12028..0x12048 - TX Message Buffer Data Field 7 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7__1(&self, n: usize) -> &Cfdtmdf7_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73768).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12028..0x12048 - TX Message Buffer Data Field 7 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf7__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf7_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73768).add(128 * n).cast()
            })
    }
    ///0x12028 - TX Message Buffer Data Field 7 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_0_1(&self) -> &Cfdtmdf7_1 {
        self.cfdtmdf7__1(0)
    }
    ///0x120a8 - TX Message Buffer Data Field 7 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_1_1(&self) -> &Cfdtmdf7_1 {
        self.cfdtmdf7__1(1)
    }
    ///0x12128 - TX Message Buffer Data Field 7 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_2_1(&self) -> &Cfdtmdf7_1 {
        self.cfdtmdf7__1(2)
    }
    ///0x121a8 - TX Message Buffer Data Field 7 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_3_1(&self) -> &Cfdtmdf7_1 {
        self.cfdtmdf7__1(3)
    }
    ///0x12228 - TX Message Buffer Data Field 7 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_4_1(&self) -> &Cfdtmdf7_1 {
        self.cfdtmdf7__1(4)
    }
    ///0x122a8 - TX Message Buffer Data Field 7 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_5_1(&self) -> &Cfdtmdf7_1 {
        self.cfdtmdf7__1(5)
    }
    ///0x12328 - TX Message Buffer Data Field 7 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_6_1(&self) -> &Cfdtmdf7_1 {
        self.cfdtmdf7__1(6)
    }
    ///0x123a8 - TX Message Buffer Data Field 7 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_7_1(&self) -> &Cfdtmdf7_1 {
        self.cfdtmdf7__1(7)
    }
    ///0x1202c..0x1204c - TX Message Buffer Data Field 8 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8__1(&self, n: usize) -> &Cfdtmdf8_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73772).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x1202c..0x1204c - TX Message Buffer Data Field 8 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf8__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf8_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73772).add(128 * n).cast()
            })
    }
    ///0x1202c - TX Message Buffer Data Field 8 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_0_1(&self) -> &Cfdtmdf8_1 {
        self.cfdtmdf8__1(0)
    }
    ///0x120ac - TX Message Buffer Data Field 8 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_1_1(&self) -> &Cfdtmdf8_1 {
        self.cfdtmdf8__1(1)
    }
    ///0x1212c - TX Message Buffer Data Field 8 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_2_1(&self) -> &Cfdtmdf8_1 {
        self.cfdtmdf8__1(2)
    }
    ///0x121ac - TX Message Buffer Data Field 8 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_3_1(&self) -> &Cfdtmdf8_1 {
        self.cfdtmdf8__1(3)
    }
    ///0x1222c - TX Message Buffer Data Field 8 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_4_1(&self) -> &Cfdtmdf8_1 {
        self.cfdtmdf8__1(4)
    }
    ///0x122ac - TX Message Buffer Data Field 8 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_5_1(&self) -> &Cfdtmdf8_1 {
        self.cfdtmdf8__1(5)
    }
    ///0x1232c - TX Message Buffer Data Field 8 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_6_1(&self) -> &Cfdtmdf8_1 {
        self.cfdtmdf8__1(6)
    }
    ///0x123ac - TX Message Buffer Data Field 8 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_7_1(&self) -> &Cfdtmdf8_1 {
        self.cfdtmdf8__1(7)
    }
    ///0x12030..0x12050 - TX Message Buffer Data Field 9 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9__1(&self, n: usize) -> &Cfdtmdf9_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73776).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12030..0x12050 - TX Message Buffer Data Field 9 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf9__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf9_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73776).add(128 * n).cast()
            })
    }
    ///0x12030 - TX Message Buffer Data Field 9 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_0_1(&self) -> &Cfdtmdf9_1 {
        self.cfdtmdf9__1(0)
    }
    ///0x120b0 - TX Message Buffer Data Field 9 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_1_1(&self) -> &Cfdtmdf9_1 {
        self.cfdtmdf9__1(1)
    }
    ///0x12130 - TX Message Buffer Data Field 9 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_2_1(&self) -> &Cfdtmdf9_1 {
        self.cfdtmdf9__1(2)
    }
    ///0x121b0 - TX Message Buffer Data Field 9 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_3_1(&self) -> &Cfdtmdf9_1 {
        self.cfdtmdf9__1(3)
    }
    ///0x12230 - TX Message Buffer Data Field 9 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_4_1(&self) -> &Cfdtmdf9_1 {
        self.cfdtmdf9__1(4)
    }
    ///0x122b0 - TX Message Buffer Data Field 9 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_5_1(&self) -> &Cfdtmdf9_1 {
        self.cfdtmdf9__1(5)
    }
    ///0x12330 - TX Message Buffer Data Field 9 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_6_1(&self) -> &Cfdtmdf9_1 {
        self.cfdtmdf9__1(6)
    }
    ///0x123b0 - TX Message Buffer Data Field 9 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_7_1(&self) -> &Cfdtmdf9_1 {
        self.cfdtmdf9__1(7)
    }
    ///0x12034..0x12054 - TX Message Buffer Data Field 10 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10__1(&self, n: usize) -> &Cfdtmdf10_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73780).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12034..0x12054 - TX Message Buffer Data Field 10 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf10__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf10_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73780).add(128 * n).cast()
            })
    }
    ///0x12034 - TX Message Buffer Data Field 10 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_0_1(&self) -> &Cfdtmdf10_1 {
        self.cfdtmdf10__1(0)
    }
    ///0x120b4 - TX Message Buffer Data Field 10 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_1_1(&self) -> &Cfdtmdf10_1 {
        self.cfdtmdf10__1(1)
    }
    ///0x12134 - TX Message Buffer Data Field 10 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_2_1(&self) -> &Cfdtmdf10_1 {
        self.cfdtmdf10__1(2)
    }
    ///0x121b4 - TX Message Buffer Data Field 10 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_3_1(&self) -> &Cfdtmdf10_1 {
        self.cfdtmdf10__1(3)
    }
    ///0x12234 - TX Message Buffer Data Field 10 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_4_1(&self) -> &Cfdtmdf10_1 {
        self.cfdtmdf10__1(4)
    }
    ///0x122b4 - TX Message Buffer Data Field 10 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_5_1(&self) -> &Cfdtmdf10_1 {
        self.cfdtmdf10__1(5)
    }
    ///0x12334 - TX Message Buffer Data Field 10 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_6_1(&self) -> &Cfdtmdf10_1 {
        self.cfdtmdf10__1(6)
    }
    ///0x123b4 - TX Message Buffer Data Field 10 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_7_1(&self) -> &Cfdtmdf10_1 {
        self.cfdtmdf10__1(7)
    }
    ///0x12038..0x12058 - TX Message Buffer Data Field 11 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11__1(&self, n: usize) -> &Cfdtmdf11_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73784).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12038..0x12058 - TX Message Buffer Data Field 11 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf11__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf11_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73784).add(128 * n).cast()
            })
    }
    ///0x12038 - TX Message Buffer Data Field 11 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_0_1(&self) -> &Cfdtmdf11_1 {
        self.cfdtmdf11__1(0)
    }
    ///0x120b8 - TX Message Buffer Data Field 11 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_1_1(&self) -> &Cfdtmdf11_1 {
        self.cfdtmdf11__1(1)
    }
    ///0x12138 - TX Message Buffer Data Field 11 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_2_1(&self) -> &Cfdtmdf11_1 {
        self.cfdtmdf11__1(2)
    }
    ///0x121b8 - TX Message Buffer Data Field 11 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_3_1(&self) -> &Cfdtmdf11_1 {
        self.cfdtmdf11__1(3)
    }
    ///0x12238 - TX Message Buffer Data Field 11 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_4_1(&self) -> &Cfdtmdf11_1 {
        self.cfdtmdf11__1(4)
    }
    ///0x122b8 - TX Message Buffer Data Field 11 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_5_1(&self) -> &Cfdtmdf11_1 {
        self.cfdtmdf11__1(5)
    }
    ///0x12338 - TX Message Buffer Data Field 11 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_6_1(&self) -> &Cfdtmdf11_1 {
        self.cfdtmdf11__1(6)
    }
    ///0x123b8 - TX Message Buffer Data Field 11 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_7_1(&self) -> &Cfdtmdf11_1 {
        self.cfdtmdf11__1(7)
    }
    ///0x1203c..0x1205c - TX Message Buffer Data Field 12 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12__1(&self, n: usize) -> &Cfdtmdf12_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73788).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x1203c..0x1205c - TX Message Buffer Data Field 12 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf12__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf12_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73788).add(128 * n).cast()
            })
    }
    ///0x1203c - TX Message Buffer Data Field 12 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_0_1(&self) -> &Cfdtmdf12_1 {
        self.cfdtmdf12__1(0)
    }
    ///0x120bc - TX Message Buffer Data Field 12 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_1_1(&self) -> &Cfdtmdf12_1 {
        self.cfdtmdf12__1(1)
    }
    ///0x1213c - TX Message Buffer Data Field 12 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_2_1(&self) -> &Cfdtmdf12_1 {
        self.cfdtmdf12__1(2)
    }
    ///0x121bc - TX Message Buffer Data Field 12 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_3_1(&self) -> &Cfdtmdf12_1 {
        self.cfdtmdf12__1(3)
    }
    ///0x1223c - TX Message Buffer Data Field 12 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_4_1(&self) -> &Cfdtmdf12_1 {
        self.cfdtmdf12__1(4)
    }
    ///0x122bc - TX Message Buffer Data Field 12 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_5_1(&self) -> &Cfdtmdf12_1 {
        self.cfdtmdf12__1(5)
    }
    ///0x1233c - TX Message Buffer Data Field 12 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_6_1(&self) -> &Cfdtmdf12_1 {
        self.cfdtmdf12__1(6)
    }
    ///0x123bc - TX Message Buffer Data Field 12 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_7_1(&self) -> &Cfdtmdf12_1 {
        self.cfdtmdf12__1(7)
    }
    ///0x12040..0x12060 - TX Message Buffer Data Field 13 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13__1(&self, n: usize) -> &Cfdtmdf13_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73792).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12040..0x12060 - TX Message Buffer Data Field 13 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf13__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf13_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73792).add(128 * n).cast()
            })
    }
    ///0x12040 - TX Message Buffer Data Field 13 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_0_1(&self) -> &Cfdtmdf13_1 {
        self.cfdtmdf13__1(0)
    }
    ///0x120c0 - TX Message Buffer Data Field 13 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_1_1(&self) -> &Cfdtmdf13_1 {
        self.cfdtmdf13__1(1)
    }
    ///0x12140 - TX Message Buffer Data Field 13 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_2_1(&self) -> &Cfdtmdf13_1 {
        self.cfdtmdf13__1(2)
    }
    ///0x121c0 - TX Message Buffer Data Field 13 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_3_1(&self) -> &Cfdtmdf13_1 {
        self.cfdtmdf13__1(3)
    }
    ///0x12240 - TX Message Buffer Data Field 13 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_4_1(&self) -> &Cfdtmdf13_1 {
        self.cfdtmdf13__1(4)
    }
    ///0x122c0 - TX Message Buffer Data Field 13 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_5_1(&self) -> &Cfdtmdf13_1 {
        self.cfdtmdf13__1(5)
    }
    ///0x12340 - TX Message Buffer Data Field 13 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_6_1(&self) -> &Cfdtmdf13_1 {
        self.cfdtmdf13__1(6)
    }
    ///0x123c0 - TX Message Buffer Data Field 13 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_7_1(&self) -> &Cfdtmdf13_1 {
        self.cfdtmdf13__1(7)
    }
    ///0x12044..0x12064 - TX Message Buffer Data Field 14 Register %s Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14__1(&self, n: usize) -> &Cfdtmdf14_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73796).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12044..0x12064 - TX Message Buffer Data Field 14 Register %s Channel 1
    #[inline(always)]
    pub fn cfdtmdf14__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf14_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73796).add(128 * n).cast()
            })
    }
    ///0x12044 - TX Message Buffer Data Field 14 Register 0 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_0_1(&self) -> &Cfdtmdf14_1 {
        self.cfdtmdf14__1(0)
    }
    ///0x120c4 - TX Message Buffer Data Field 14 Register 1 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_1_1(&self) -> &Cfdtmdf14_1 {
        self.cfdtmdf14__1(1)
    }
    ///0x12144 - TX Message Buffer Data Field 14 Register 2 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_2_1(&self) -> &Cfdtmdf14_1 {
        self.cfdtmdf14__1(2)
    }
    ///0x121c4 - TX Message Buffer Data Field 14 Register 3 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_3_1(&self) -> &Cfdtmdf14_1 {
        self.cfdtmdf14__1(3)
    }
    ///0x12244 - TX Message Buffer Data Field 14 Register 4 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_4_1(&self) -> &Cfdtmdf14_1 {
        self.cfdtmdf14__1(4)
    }
    ///0x122c4 - TX Message Buffer Data Field 14 Register 5 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_5_1(&self) -> &Cfdtmdf14_1 {
        self.cfdtmdf14__1(5)
    }
    ///0x12344 - TX Message Buffer Data Field 14 Register 6 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_6_1(&self) -> &Cfdtmdf14_1 {
        self.cfdtmdf14__1(6)
    }
    ///0x123c4 - TX Message Buffer Data Field 14 Register 7 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_7_1(&self) -> &Cfdtmdf14_1 {
        self.cfdtmdf14__1(7)
    }
    ///0x12048..0x12068 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15__1(&self, n: usize) -> &Cfdtmdf15_1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self).cast::<u8>().add(73800).add(128 * n).cast()
        }
    }
    ///Iterator for array of:
    ///0x12048..0x12068 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub fn cfdtmdf15__1_iter(&self) -> impl Iterator<Item = &Cfdtmdf15_1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(73800).add(128 * n).cast()
            })
    }
    ///0x12048 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_0_1(&self) -> &Cfdtmdf15_1 {
        self.cfdtmdf15__1(0)
    }
    ///0x120c8 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_1_1(&self) -> &Cfdtmdf15_1 {
        self.cfdtmdf15__1(1)
    }
    ///0x12148 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_2_1(&self) -> &Cfdtmdf15_1 {
        self.cfdtmdf15__1(2)
    }
    ///0x121c8 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_3_1(&self) -> &Cfdtmdf15_1 {
        self.cfdtmdf15__1(3)
    }
    ///0x12248 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_4_1(&self) -> &Cfdtmdf15_1 {
        self.cfdtmdf15__1(4)
    }
    ///0x122c8 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_5_1(&self) -> &Cfdtmdf15_1 {
        self.cfdtmdf15__1(5)
    }
    ///0x12348 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_6_1(&self) -> &Cfdtmdf15_1 {
        self.cfdtmdf15__1(6)
    }
    ///0x123c8 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_7_1(&self) -> &Cfdtmdf15_1 {
        self.cfdtmdf15__1(7)
    }
    ///0x13000 - TX Message Buffer ID Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmid32_1(&self) -> &Cfdtmid1 {
        &self.cfdtmid32_1
    }
    ///0x13004 - TX Message Buffer Pointer Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr32_1(&self) -> &Cfdtmptr1 {
        &self.cfdtmptr32_1
    }
    ///0x13008 - TX Message Buffer CANFD Control Register 32 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr32_1(&self) -> &Cfdtmfdctr1 {
        &self.cfdtmfdctr32_1
    }
    ///0x1300c - TX Message Buffer Data Field 0 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_32_1(&self) -> &Cfdtmdf0_1 {
        &self.cfdtmdf0_32_1
    }
    ///0x13010 - TX Message Buffer Data Field 1 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_32_1(&self) -> &Cfdtmdf1_1 {
        &self.cfdtmdf1_32_1
    }
    ///0x13014 - TX Message Buffer Data Field 2 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_32_1(&self) -> &Cfdtmdf2_1 {
        &self.cfdtmdf2_32_1
    }
    ///0x13018 - TX Message Buffer Data Field 3 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_32_1(&self) -> &Cfdtmdf3_1 {
        &self.cfdtmdf3_32_1
    }
    ///0x1301c - TX Message Buffer Data Field 4 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_32_1(&self) -> &Cfdtmdf4_1 {
        &self.cfdtmdf4_32_1
    }
    ///0x13020 - TX Message Buffer Data Field 5 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_32_1(&self) -> &Cfdtmdf5_1 {
        &self.cfdtmdf5_32_1
    }
    ///0x13024 - TX Message Buffer Data Field 6 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_32_1(&self) -> &Cfdtmdf6_1 {
        &self.cfdtmdf6_32_1
    }
    ///0x13028 - TX Message Buffer Data Field 7 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_32_1(&self) -> &Cfdtmdf7_1 {
        &self.cfdtmdf7_32_1
    }
    ///0x1302c - TX Message Buffer Data Field 8 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_32_1(&self) -> &Cfdtmdf8_1 {
        &self.cfdtmdf8_32_1
    }
    ///0x13030 - TX Message Buffer Data Field 9 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_32_1(&self) -> &Cfdtmdf9_1 {
        &self.cfdtmdf9_32_1
    }
    ///0x13034 - TX Message Buffer Data Field 10 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_32_1(&self) -> &Cfdtmdf10_1 {
        &self.cfdtmdf10_32_1
    }
    ///0x13038 - TX Message Buffer Data Field 11 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_32_1(&self) -> &Cfdtmdf11_1 {
        &self.cfdtmdf11_32_1
    }
    ///0x1303c - TX Message Buffer Data Field 12 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_32_1(&self) -> &Cfdtmdf12_1 {
        &self.cfdtmdf12_32_1
    }
    ///0x13040 - TX Message Buffer Data Field 13 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_32_1(&self) -> &Cfdtmdf13_1 {
        &self.cfdtmdf13_32_1
    }
    ///0x13044 - TX Message Buffer Data Field 14 Register 32 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_32_1(&self) -> &Cfdtmdf14_1 {
        &self.cfdtmdf14_32_1
    }
    ///0x13048 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_32_1(&self) -> &Cfdtmdf15_1 {
        &self.cfdtmdf15_32_1
    }
    ///0x13000 - TX Message Buffer ID Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmid33_1(&self) -> &Cfdtmid1 {
        &self.cfdtmid33_1
    }
    ///0x13004 - TX Message Buffer Pointer Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr33_1(&self) -> &Cfdtmptr1 {
        &self.cfdtmptr33_1
    }
    ///0x13008 - TX Message Buffer CANFD Control Register 33 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr33_1(&self) -> &Cfdtmfdctr1 {
        &self.cfdtmfdctr33_1
    }
    ///0x1300c - TX Message Buffer Data Field 0 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_33_1(&self) -> &Cfdtmdf0_1 {
        &self.cfdtmdf0_33_1
    }
    ///0x13010 - TX Message Buffer Data Field 1 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_33_1(&self) -> &Cfdtmdf1_1 {
        &self.cfdtmdf1_33_1
    }
    ///0x13014 - TX Message Buffer Data Field 2 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_33_1(&self) -> &Cfdtmdf2_1 {
        &self.cfdtmdf2_33_1
    }
    ///0x13018 - TX Message Buffer Data Field 3 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_33_1(&self) -> &Cfdtmdf3_1 {
        &self.cfdtmdf3_33_1
    }
    ///0x1301c - TX Message Buffer Data Field 4 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_33_1(&self) -> &Cfdtmdf4_1 {
        &self.cfdtmdf4_33_1
    }
    ///0x13020 - TX Message Buffer Data Field 5 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_33_1(&self) -> &Cfdtmdf5_1 {
        &self.cfdtmdf5_33_1
    }
    ///0x13024 - TX Message Buffer Data Field 6 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_33_1(&self) -> &Cfdtmdf6_1 {
        &self.cfdtmdf6_33_1
    }
    ///0x13028 - TX Message Buffer Data Field 7 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_33_1(&self) -> &Cfdtmdf7_1 {
        &self.cfdtmdf7_33_1
    }
    ///0x1302c - TX Message Buffer Data Field 8 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_33_1(&self) -> &Cfdtmdf8_1 {
        &self.cfdtmdf8_33_1
    }
    ///0x13030 - TX Message Buffer Data Field 9 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_33_1(&self) -> &Cfdtmdf9_1 {
        &self.cfdtmdf9_33_1
    }
    ///0x13034 - TX Message Buffer Data Field 10 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_33_1(&self) -> &Cfdtmdf10_1 {
        &self.cfdtmdf10_33_1
    }
    ///0x13038 - TX Message Buffer Data Field 11 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_33_1(&self) -> &Cfdtmdf11_1 {
        &self.cfdtmdf11_33_1
    }
    ///0x1303c - TX Message Buffer Data Field 12 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_33_1(&self) -> &Cfdtmdf12_1 {
        &self.cfdtmdf12_33_1
    }
    ///0x13040 - TX Message Buffer Data Field 13 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_33_1(&self) -> &Cfdtmdf13_1 {
        &self.cfdtmdf13_33_1
    }
    ///0x13044 - TX Message Buffer Data Field 14 Register 33 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_33_1(&self) -> &Cfdtmdf14_1 {
        &self.cfdtmdf14_33_1
    }
    ///0x13048 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_33_1(&self) -> &Cfdtmdf15_1 {
        &self.cfdtmdf15_33_1
    }
    ///0x13000 - TX Message Buffer ID Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmid34_1(&self) -> &Cfdtmid1 {
        &self.cfdtmid34_1
    }
    ///0x13004 - TX Message Buffer Pointer Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr34_1(&self) -> &Cfdtmptr1 {
        &self.cfdtmptr34_1
    }
    ///0x13008 - TX Message Buffer CANFD Control Register 34 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr34_1(&self) -> &Cfdtmfdctr1 {
        &self.cfdtmfdctr34_1
    }
    ///0x1300c - TX Message Buffer Data Field 0 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_34_1(&self) -> &Cfdtmdf0_1 {
        &self.cfdtmdf0_34_1
    }
    ///0x13010 - TX Message Buffer Data Field 1 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_34_1(&self) -> &Cfdtmdf1_1 {
        &self.cfdtmdf1_34_1
    }
    ///0x13014 - TX Message Buffer Data Field 2 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_34_1(&self) -> &Cfdtmdf2_1 {
        &self.cfdtmdf2_34_1
    }
    ///0x13018 - TX Message Buffer Data Field 3 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_34_1(&self) -> &Cfdtmdf3_1 {
        &self.cfdtmdf3_34_1
    }
    ///0x1301c - TX Message Buffer Data Field 4 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_34_1(&self) -> &Cfdtmdf4_1 {
        &self.cfdtmdf4_34_1
    }
    ///0x13020 - TX Message Buffer Data Field 5 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_34_1(&self) -> &Cfdtmdf5_1 {
        &self.cfdtmdf5_34_1
    }
    ///0x13024 - TX Message Buffer Data Field 6 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_34_1(&self) -> &Cfdtmdf6_1 {
        &self.cfdtmdf6_34_1
    }
    ///0x13028 - TX Message Buffer Data Field 7 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_34_1(&self) -> &Cfdtmdf7_1 {
        &self.cfdtmdf7_34_1
    }
    ///0x1302c - TX Message Buffer Data Field 8 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_34_1(&self) -> &Cfdtmdf8_1 {
        &self.cfdtmdf8_34_1
    }
    ///0x13030 - TX Message Buffer Data Field 9 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_34_1(&self) -> &Cfdtmdf9_1 {
        &self.cfdtmdf9_34_1
    }
    ///0x13034 - TX Message Buffer Data Field 10 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_34_1(&self) -> &Cfdtmdf10_1 {
        &self.cfdtmdf10_34_1
    }
    ///0x13038 - TX Message Buffer Data Field 11 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_34_1(&self) -> &Cfdtmdf11_1 {
        &self.cfdtmdf11_34_1
    }
    ///0x1303c - TX Message Buffer Data Field 12 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_34_1(&self) -> &Cfdtmdf12_1 {
        &self.cfdtmdf12_34_1
    }
    ///0x13040 - TX Message Buffer Data Field 13 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_34_1(&self) -> &Cfdtmdf13_1 {
        &self.cfdtmdf13_34_1
    }
    ///0x13044 - TX Message Buffer Data Field 14 Register 34 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_34_1(&self) -> &Cfdtmdf14_1 {
        &self.cfdtmdf14_34_1
    }
    ///0x13048 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_34_1(&self) -> &Cfdtmdf15_1 {
        &self.cfdtmdf15_34_1
    }
    ///0x13000 - TX Message Buffer ID Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmid35_1(&self) -> &Cfdtmid1 {
        &self.cfdtmid35_1
    }
    ///0x13004 - TX Message Buffer Pointer Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr35_1(&self) -> &Cfdtmptr1 {
        &self.cfdtmptr35_1
    }
    ///0x13008 - TX Message Buffer CANFD Control Register 35 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr35_1(&self) -> &Cfdtmfdctr1 {
        &self.cfdtmfdctr35_1
    }
    ///0x1300c - TX Message Buffer Data Field 0 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_35_1(&self) -> &Cfdtmdf0_1 {
        &self.cfdtmdf0_35_1
    }
    ///0x13010 - TX Message Buffer Data Field 1 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_35_1(&self) -> &Cfdtmdf1_1 {
        &self.cfdtmdf1_35_1
    }
    ///0x13014 - TX Message Buffer Data Field 2 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_35_1(&self) -> &Cfdtmdf2_1 {
        &self.cfdtmdf2_35_1
    }
    ///0x13018 - TX Message Buffer Data Field 3 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_35_1(&self) -> &Cfdtmdf3_1 {
        &self.cfdtmdf3_35_1
    }
    ///0x1301c - TX Message Buffer Data Field 4 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_35_1(&self) -> &Cfdtmdf4_1 {
        &self.cfdtmdf4_35_1
    }
    ///0x13020 - TX Message Buffer Data Field 5 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_35_1(&self) -> &Cfdtmdf5_1 {
        &self.cfdtmdf5_35_1
    }
    ///0x13024 - TX Message Buffer Data Field 6 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_35_1(&self) -> &Cfdtmdf6_1 {
        &self.cfdtmdf6_35_1
    }
    ///0x13028 - TX Message Buffer Data Field 7 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_35_1(&self) -> &Cfdtmdf7_1 {
        &self.cfdtmdf7_35_1
    }
    ///0x1302c - TX Message Buffer Data Field 8 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_35_1(&self) -> &Cfdtmdf8_1 {
        &self.cfdtmdf8_35_1
    }
    ///0x13030 - TX Message Buffer Data Field 9 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_35_1(&self) -> &Cfdtmdf9_1 {
        &self.cfdtmdf9_35_1
    }
    ///0x13034 - TX Message Buffer Data Field 10 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_35_1(&self) -> &Cfdtmdf10_1 {
        &self.cfdtmdf10_35_1
    }
    ///0x13038 - TX Message Buffer Data Field 11 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_35_1(&self) -> &Cfdtmdf11_1 {
        &self.cfdtmdf11_35_1
    }
    ///0x1303c - TX Message Buffer Data Field 12 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_35_1(&self) -> &Cfdtmdf12_1 {
        &self.cfdtmdf12_35_1
    }
    ///0x13040 - TX Message Buffer Data Field 13 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_35_1(&self) -> &Cfdtmdf13_1 {
        &self.cfdtmdf13_35_1
    }
    ///0x13044 - TX Message Buffer Data Field 14 Register 35 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_35_1(&self) -> &Cfdtmdf14_1 {
        &self.cfdtmdf14_35_1
    }
    ///0x13048 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_35_1(&self) -> &Cfdtmdf15_1 {
        &self.cfdtmdf15_35_1
    }
    ///0x13000 - TX Message Buffer ID Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmid36_1(&self) -> &Cfdtmid1 {
        &self.cfdtmid36_1
    }
    ///0x13004 - TX Message Buffer Pointer Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr36_1(&self) -> &Cfdtmptr1 {
        &self.cfdtmptr36_1
    }
    ///0x13008 - TX Message Buffer CANFD Control Register 36 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr36_1(&self) -> &Cfdtmfdctr1 {
        &self.cfdtmfdctr36_1
    }
    ///0x1300c - TX Message Buffer Data Field 0 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_36_1(&self) -> &Cfdtmdf0_1 {
        &self.cfdtmdf0_36_1
    }
    ///0x13010 - TX Message Buffer Data Field 1 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_36_1(&self) -> &Cfdtmdf1_1 {
        &self.cfdtmdf1_36_1
    }
    ///0x13014 - TX Message Buffer Data Field 2 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_36_1(&self) -> &Cfdtmdf2_1 {
        &self.cfdtmdf2_36_1
    }
    ///0x13018 - TX Message Buffer Data Field 3 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_36_1(&self) -> &Cfdtmdf3_1 {
        &self.cfdtmdf3_36_1
    }
    ///0x1301c - TX Message Buffer Data Field 4 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_36_1(&self) -> &Cfdtmdf4_1 {
        &self.cfdtmdf4_36_1
    }
    ///0x13020 - TX Message Buffer Data Field 5 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_36_1(&self) -> &Cfdtmdf5_1 {
        &self.cfdtmdf5_36_1
    }
    ///0x13024 - TX Message Buffer Data Field 6 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_36_1(&self) -> &Cfdtmdf6_1 {
        &self.cfdtmdf6_36_1
    }
    ///0x13028 - TX Message Buffer Data Field 7 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_36_1(&self) -> &Cfdtmdf7_1 {
        &self.cfdtmdf7_36_1
    }
    ///0x1302c - TX Message Buffer Data Field 8 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_36_1(&self) -> &Cfdtmdf8_1 {
        &self.cfdtmdf8_36_1
    }
    ///0x13030 - TX Message Buffer Data Field 9 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_36_1(&self) -> &Cfdtmdf9_1 {
        &self.cfdtmdf9_36_1
    }
    ///0x13034 - TX Message Buffer Data Field 10 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_36_1(&self) -> &Cfdtmdf10_1 {
        &self.cfdtmdf10_36_1
    }
    ///0x13038 - TX Message Buffer Data Field 11 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_36_1(&self) -> &Cfdtmdf11_1 {
        &self.cfdtmdf11_36_1
    }
    ///0x1303c - TX Message Buffer Data Field 12 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_36_1(&self) -> &Cfdtmdf12_1 {
        &self.cfdtmdf12_36_1
    }
    ///0x13040 - TX Message Buffer Data Field 13 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_36_1(&self) -> &Cfdtmdf13_1 {
        &self.cfdtmdf13_36_1
    }
    ///0x13044 - TX Message Buffer Data Field 14 Register 36 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_36_1(&self) -> &Cfdtmdf14_1 {
        &self.cfdtmdf14_36_1
    }
    ///0x13048 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_36_1(&self) -> &Cfdtmdf15_1 {
        &self.cfdtmdf15_36_1
    }
    ///0x13000 - TX Message Buffer ID Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmid37_1(&self) -> &Cfdtmid1 {
        &self.cfdtmid37_1
    }
    ///0x13004 - TX Message Buffer Pointer Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr37_1(&self) -> &Cfdtmptr1 {
        &self.cfdtmptr37_1
    }
    ///0x13008 - TX Message Buffer CANFD Control Register 37 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr37_1(&self) -> &Cfdtmfdctr1 {
        &self.cfdtmfdctr37_1
    }
    ///0x1300c - TX Message Buffer Data Field 0 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_37_1(&self) -> &Cfdtmdf0_1 {
        &self.cfdtmdf0_37_1
    }
    ///0x13010 - TX Message Buffer Data Field 1 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_37_1(&self) -> &Cfdtmdf1_1 {
        &self.cfdtmdf1_37_1
    }
    ///0x13014 - TX Message Buffer Data Field 2 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_37_1(&self) -> &Cfdtmdf2_1 {
        &self.cfdtmdf2_37_1
    }
    ///0x13018 - TX Message Buffer Data Field 3 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_37_1(&self) -> &Cfdtmdf3_1 {
        &self.cfdtmdf3_37_1
    }
    ///0x1301c - TX Message Buffer Data Field 4 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_37_1(&self) -> &Cfdtmdf4_1 {
        &self.cfdtmdf4_37_1
    }
    ///0x13020 - TX Message Buffer Data Field 5 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_37_1(&self) -> &Cfdtmdf5_1 {
        &self.cfdtmdf5_37_1
    }
    ///0x13024 - TX Message Buffer Data Field 6 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_37_1(&self) -> &Cfdtmdf6_1 {
        &self.cfdtmdf6_37_1
    }
    ///0x13028 - TX Message Buffer Data Field 7 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_37_1(&self) -> &Cfdtmdf7_1 {
        &self.cfdtmdf7_37_1
    }
    ///0x1302c - TX Message Buffer Data Field 8 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_37_1(&self) -> &Cfdtmdf8_1 {
        &self.cfdtmdf8_37_1
    }
    ///0x13030 - TX Message Buffer Data Field 9 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_37_1(&self) -> &Cfdtmdf9_1 {
        &self.cfdtmdf9_37_1
    }
    ///0x13034 - TX Message Buffer Data Field 10 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_37_1(&self) -> &Cfdtmdf10_1 {
        &self.cfdtmdf10_37_1
    }
    ///0x13038 - TX Message Buffer Data Field 11 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_37_1(&self) -> &Cfdtmdf11_1 {
        &self.cfdtmdf11_37_1
    }
    ///0x1303c - TX Message Buffer Data Field 12 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_37_1(&self) -> &Cfdtmdf12_1 {
        &self.cfdtmdf12_37_1
    }
    ///0x13040 - TX Message Buffer Data Field 13 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_37_1(&self) -> &Cfdtmdf13_1 {
        &self.cfdtmdf13_37_1
    }
    ///0x13044 - TX Message Buffer Data Field 14 Register 37 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_37_1(&self) -> &Cfdtmdf14_1 {
        &self.cfdtmdf14_37_1
    }
    ///0x13048 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_37_1(&self) -> &Cfdtmdf15_1 {
        &self.cfdtmdf15_37_1
    }
    ///0x13000 - TX Message Buffer ID Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmid38_1(&self) -> &Cfdtmid1 {
        &self.cfdtmid38_1
    }
    ///0x13004 - TX Message Buffer Pointer Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr38_1(&self) -> &Cfdtmptr1 {
        &self.cfdtmptr38_1
    }
    ///0x13008 - TX Message Buffer CANFD Control Register 38 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr38_1(&self) -> &Cfdtmfdctr1 {
        &self.cfdtmfdctr38_1
    }
    ///0x1300c - TX Message Buffer Data Field 0 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_38_1(&self) -> &Cfdtmdf0_1 {
        &self.cfdtmdf0_38_1
    }
    ///0x13010 - TX Message Buffer Data Field 1 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_38_1(&self) -> &Cfdtmdf1_1 {
        &self.cfdtmdf1_38_1
    }
    ///0x13014 - TX Message Buffer Data Field 2 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_38_1(&self) -> &Cfdtmdf2_1 {
        &self.cfdtmdf2_38_1
    }
    ///0x13018 - TX Message Buffer Data Field 3 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_38_1(&self) -> &Cfdtmdf3_1 {
        &self.cfdtmdf3_38_1
    }
    ///0x1301c - TX Message Buffer Data Field 4 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_38_1(&self) -> &Cfdtmdf4_1 {
        &self.cfdtmdf4_38_1
    }
    ///0x13020 - TX Message Buffer Data Field 5 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_38_1(&self) -> &Cfdtmdf5_1 {
        &self.cfdtmdf5_38_1
    }
    ///0x13024 - TX Message Buffer Data Field 6 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_38_1(&self) -> &Cfdtmdf6_1 {
        &self.cfdtmdf6_38_1
    }
    ///0x13028 - TX Message Buffer Data Field 7 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_38_1(&self) -> &Cfdtmdf7_1 {
        &self.cfdtmdf7_38_1
    }
    ///0x1302c - TX Message Buffer Data Field 8 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_38_1(&self) -> &Cfdtmdf8_1 {
        &self.cfdtmdf8_38_1
    }
    ///0x13030 - TX Message Buffer Data Field 9 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_38_1(&self) -> &Cfdtmdf9_1 {
        &self.cfdtmdf9_38_1
    }
    ///0x13034 - TX Message Buffer Data Field 10 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_38_1(&self) -> &Cfdtmdf10_1 {
        &self.cfdtmdf10_38_1
    }
    ///0x13038 - TX Message Buffer Data Field 11 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_38_1(&self) -> &Cfdtmdf11_1 {
        &self.cfdtmdf11_38_1
    }
    ///0x1303c - TX Message Buffer Data Field 12 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_38_1(&self) -> &Cfdtmdf12_1 {
        &self.cfdtmdf12_38_1
    }
    ///0x13040 - TX Message Buffer Data Field 13 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_38_1(&self) -> &Cfdtmdf13_1 {
        &self.cfdtmdf13_38_1
    }
    ///0x13044 - TX Message Buffer Data Field 14 Register 38 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_38_1(&self) -> &Cfdtmdf14_1 {
        &self.cfdtmdf14_38_1
    }
    ///0x13048 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_38_1(&self) -> &Cfdtmdf15_1 {
        &self.cfdtmdf15_38_1
    }
    ///0x13000 - TX Message Buffer ID Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmid39_1(&self) -> &Cfdtmid1 {
        &self.cfdtmid39_1
    }
    ///0x13004 - TX Message Buffer Pointer Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmptr39_1(&self) -> &Cfdtmptr1 {
        &self.cfdtmptr39_1
    }
    ///0x13008 - TX Message Buffer CANFD Control Register 39 Channel i
    #[inline(always)]
    pub const fn cfdtmfdctr39_1(&self) -> &Cfdtmfdctr1 {
        &self.cfdtmfdctr39_1
    }
    ///0x1300c - TX Message Buffer Data Field 0 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf0_39_1(&self) -> &Cfdtmdf0_1 {
        &self.cfdtmdf0_39_1
    }
    ///0x13010 - TX Message Buffer Data Field 1 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf1_39_1(&self) -> &Cfdtmdf1_1 {
        &self.cfdtmdf1_39_1
    }
    ///0x13014 - TX Message Buffer Data Field 2 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf2_39_1(&self) -> &Cfdtmdf2_1 {
        &self.cfdtmdf2_39_1
    }
    ///0x13018 - TX Message Buffer Data Field 3 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf3_39_1(&self) -> &Cfdtmdf3_1 {
        &self.cfdtmdf3_39_1
    }
    ///0x1301c - TX Message Buffer Data Field 4 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf4_39_1(&self) -> &Cfdtmdf4_1 {
        &self.cfdtmdf4_39_1
    }
    ///0x13020 - TX Message Buffer Data Field 5 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf5_39_1(&self) -> &Cfdtmdf5_1 {
        &self.cfdtmdf5_39_1
    }
    ///0x13024 - TX Message Buffer Data Field 6 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf6_39_1(&self) -> &Cfdtmdf6_1 {
        &self.cfdtmdf6_39_1
    }
    ///0x13028 - TX Message Buffer Data Field 7 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf7_39_1(&self) -> &Cfdtmdf7_1 {
        &self.cfdtmdf7_39_1
    }
    ///0x1302c - TX Message Buffer Data Field 8 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf8_39_1(&self) -> &Cfdtmdf8_1 {
        &self.cfdtmdf8_39_1
    }
    ///0x13030 - TX Message Buffer Data Field 9 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf9_39_1(&self) -> &Cfdtmdf9_1 {
        &self.cfdtmdf9_39_1
    }
    ///0x13034 - TX Message Buffer Data Field 10 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf10_39_1(&self) -> &Cfdtmdf10_1 {
        &self.cfdtmdf10_39_1
    }
    ///0x13038 - TX Message Buffer Data Field 11 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf11_39_1(&self) -> &Cfdtmdf11_1 {
        &self.cfdtmdf11_39_1
    }
    ///0x1303c - TX Message Buffer Data Field 12 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf12_39_1(&self) -> &Cfdtmdf12_1 {
        &self.cfdtmdf12_39_1
    }
    ///0x13040 - TX Message Buffer Data Field 13 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf13_39_1(&self) -> &Cfdtmdf13_1 {
        &self.cfdtmdf13_39_1
    }
    ///0x13044 - TX Message Buffer Data Field 14 Register 39 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf14_39_1(&self) -> &Cfdtmdf14_1 {
        &self.cfdtmdf14_39_1
    }
    ///0x13048 - TX Message Buffer Data Field X Register 15 Channel 1
    #[inline(always)]
    pub const fn cfdtmdf15_39_1(&self) -> &Cfdtmdf15_1 {
        &self.cfdtmdf15_39_1
    }
}
/**CFDCNCFG (rw) register accessor: Channel %s Nominal Bitrate Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdcncfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcncfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcncfg`] module*/
#[doc(alias = "CFDCNCFG")]
pub type Cfdcncfg = crate::Reg<cfdcncfg::CfdcncfgSpec>;
///Channel %s Nominal Bitrate Configuration Register
pub mod cfdcncfg;
/**CFDCCTR (rw) register accessor: Channel %s Control Registers

You can [`read`](crate::Reg::read) this register and get [`cfdcctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcctr`] module*/
#[doc(alias = "CFDCCTR")]
pub type Cfdcctr = crate::Reg<cfdcctr::CfdcctrSpec>;
///Channel %s Control Registers
pub mod cfdcctr;
/**CFDCSTS (rw) register accessor: Channel %s Status Registers

You can [`read`](crate::Reg::read) this register and get [`cfdcsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcsts`] module*/
#[doc(alias = "CFDCSTS")]
pub type Cfdcsts = crate::Reg<cfdcsts::CfdcstsSpec>;
///Channel %s Status Registers
pub mod cfdcsts;
/**CFDCERFL (rw) register accessor: Channel %s Error Flag Registers

You can [`read`](crate::Reg::read) this register and get [`cfdcerfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcerfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcerfl`] module*/
#[doc(alias = "CFDCERFL")]
pub type Cfdcerfl = crate::Reg<cfdcerfl::CfdcerflSpec>;
///Channel %s Error Flag Registers
pub mod cfdcerfl;
/**CFDGCFG (rw) register accessor: Global Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdgcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgcfg`] module*/
#[doc(alias = "CFDGCFG")]
pub type Cfdgcfg = crate::Reg<cfdgcfg::CfdgcfgSpec>;
///Global Configuration Register
pub mod cfdgcfg;
/**CFDGCTR (rw) register accessor: Global Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdgctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgctr`] module*/
#[doc(alias = "CFDGCTR")]
pub type Cfdgctr = crate::Reg<cfdgctr::CfdgctrSpec>;
///Global Control Register
pub mod cfdgctr;
/**CFDGSTS (r) register accessor: Global Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdgsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgsts`] module*/
#[doc(alias = "CFDGSTS")]
pub type Cfdgsts = crate::Reg<cfdgsts::CfdgstsSpec>;
///Global Status Register
pub mod cfdgsts;
/**CFDGERFL (rw) register accessor: Global Error Flag Register

You can [`read`](crate::Reg::read) this register and get [`cfdgerfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgerfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgerfl`] module*/
#[doc(alias = "CFDGERFL")]
pub type Cfdgerfl = crate::Reg<cfdgerfl::CfdgerflSpec>;
///Global Error Flag Register
pub mod cfdgerfl;
/**CFDGTSC (r) register accessor: Global Timestamp Counter Register

You can [`read`](crate::Reg::read) this register and get [`cfdgtsc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgtsc`] module*/
#[doc(alias = "CFDGTSC")]
pub type Cfdgtsc = crate::Reg<cfdgtsc::CfdgtscSpec>;
///Global Timestamp Counter Register
pub mod cfdgtsc;
/**CFDGAFLECTR (rw) register accessor: Global Acceptance Filter List Entry Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdgaflectr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflectr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgaflectr`] module*/
#[doc(alias = "CFDGAFLECTR")]
pub type Cfdgaflectr = crate::Reg<cfdgaflectr::CfdgaflectrSpec>;
///Global Acceptance Filter List Entry Control Register
pub mod cfdgaflectr;
/**CFDGAFLCFG0 (rw) register accessor: Global Acceptance Filter List Configuration Register 0

You can [`read`](crate::Reg::read) this register and get [`cfdgaflcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgaflcfg0`] module*/
#[doc(alias = "CFDGAFLCFG0")]
pub type Cfdgaflcfg0 = crate::Reg<cfdgaflcfg0::Cfdgaflcfg0Spec>;
///Global Acceptance Filter List Configuration Register 0
pub mod cfdgaflcfg0;
/**CFDRMNB (rw) register accessor: RX Message Buffer Number Register

You can [`read`](crate::Reg::read) this register and get [`cfdrmnb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrmnb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmnb`] module*/
#[doc(alias = "CFDRMNB")]
pub type Cfdrmnb = crate::Reg<cfdrmnb::CfdrmnbSpec>;
///RX Message Buffer Number Register
pub mod cfdrmnb;
/**CFDRMND0 (rw) register accessor: RX Message Buffer New Data Register 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmnd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrmnd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmnd0`] module*/
#[doc(alias = "CFDRMND0")]
pub type Cfdrmnd0 = crate::Reg<cfdrmnd0::Cfdrmnd0Spec>;
///RX Message Buffer New Data Register 0
pub mod cfdrmnd0;
/**CFDRFCC (rw) register accessor: RX FIFO Configuration/Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfcc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrfcc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfcc`] module*/
#[doc(alias = "CFDRFCC")]
pub type Cfdrfcc = crate::Reg<cfdrfcc::CfdrfccSpec>;
///RX FIFO Configuration/Control Registers %s
pub mod cfdrfcc;
/**CFDRFSTS (rw) register accessor: RX FIFO Status Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrfsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfsts`] module*/
#[doc(alias = "CFDRFSTS")]
pub type Cfdrfsts = crate::Reg<cfdrfsts::CfdrfstsSpec>;
///RX FIFO Status Registers %s
pub mod cfdrfsts;
/**CFDRFPCTR (rw) register accessor: RX FIFO Pointer Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfpctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrfpctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfpctr`] module*/
#[doc(alias = "CFDRFPCTR")]
pub type Cfdrfpctr = crate::Reg<cfdrfpctr::CfdrfpctrSpec>;
///RX FIFO Pointer Control Registers %s
pub mod cfdrfpctr;
/**CFDCFCC (rw) register accessor: Common FIFO Configuration/Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdcfcc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfcc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfcc`] module*/
#[doc(alias = "CFDCFCC")]
pub type Cfdcfcc = crate::Reg<cfdcfcc::CfdcfccSpec>;
///Common FIFO Configuration/Control Registers %s
pub mod cfdcfcc;
/**CFDCFCCE (rw) register accessor: Common FIFO Configuration/Control Enhancement Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdcfcce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfcce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfcce`] module*/
#[doc(alias = "CFDCFCCE")]
pub type Cfdcfcce = crate::Reg<cfdcfcce::CfdcfcceSpec>;
///Common FIFO Configuration/Control Enhancement Registers %s
pub mod cfdcfcce;
/**CFDCFSTS (rw) register accessor: Common FIFO Status Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdcfsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfsts`] module*/
#[doc(alias = "CFDCFSTS")]
pub type Cfdcfsts = crate::Reg<cfdcfsts::CfdcfstsSpec>;
///Common FIFO Status Registers %s
pub mod cfdcfsts;
/**CFDCFPCTR (rw) register accessor: Common FIFO Pointer Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdcfpctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfpctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfpctr`] module*/
#[doc(alias = "CFDCFPCTR")]
pub type Cfdcfpctr = crate::Reg<cfdcfpctr::CfdcfpctrSpec>;
///Common FIFO Pointer Control Registers %s
pub mod cfdcfpctr;
/**CFDFESTS (r) register accessor: FIFO Empty Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdfests::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdfests`] module*/
#[doc(alias = "CFDFESTS")]
pub type Cfdfests = crate::Reg<cfdfests::CfdfestsSpec>;
///FIFO Empty Status Register
pub mod cfdfests;
/**CFDFFSTS (r) register accessor: FIFO Full Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdffsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdffsts`] module*/
#[doc(alias = "CFDFFSTS")]
pub type Cfdffsts = crate::Reg<cfdffsts::CfdffstsSpec>;
///FIFO Full Status Register
pub mod cfdffsts;
/**CFDFMSTS (r) register accessor: FIFO Message Lost Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdfmsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdfmsts`] module*/
#[doc(alias = "CFDFMSTS")]
pub type Cfdfmsts = crate::Reg<cfdfmsts::CfdfmstsSpec>;
///FIFO Message Lost Status Register
pub mod cfdfmsts;
/**CFDRFISTS (rw) register accessor: RX FIFO Interrupt Flag Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdrfists::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrfists::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfists`] module*/
#[doc(alias = "CFDRFISTS")]
pub type Cfdrfists = crate::Reg<cfdrfists::CfdrfistsSpec>;
///RX FIFO Interrupt Flag Status Register
pub mod cfdrfists;
/**CFDCFRISTS (rw) register accessor: Common FIFO RX Interrupt Flag Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfrists::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfrists::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfrists`] module*/
#[doc(alias = "CFDCFRISTS")]
pub type Cfdcfrists = crate::Reg<cfdcfrists::CfdcfristsSpec>;
///Common FIFO RX Interrupt Flag Status Register
pub mod cfdcfrists;
/**CFDCFTISTS (rw) register accessor: Common FIFO TX Interrupt Flag Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcftists::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcftists::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcftists`] module*/
#[doc(alias = "CFDCFTISTS")]
pub type Cfdcftists = crate::Reg<cfdcftists::CfdcftistsSpec>;
///Common FIFO TX Interrupt Flag Status Register
pub mod cfdcftists;
/**CFDCFOFRISTS (r) register accessor: Common FIFO One Frame RX Interrupt Flag Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfofrists::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfofrists`] module*/
#[doc(alias = "CFDCFOFRISTS")]
pub type Cfdcfofrists = crate::Reg<cfdcfofrists::CfdcfofristsSpec>;
///Common FIFO One Frame RX Interrupt Flag Status Register
pub mod cfdcfofrists;
/**CFDCFOFTISTS (r) register accessor: Common FIFO One Frame TX Interrupt Flag Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfoftists::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfoftists`] module*/
#[doc(alias = "CFDCFOFTISTS")]
pub type Cfdcfoftists = crate::Reg<cfdcfoftists::CfdcfoftistsSpec>;
///Common FIFO One Frame TX Interrupt Flag Status Register
pub mod cfdcfoftists;
/**CFDCFMOWSTS (r) register accessor: Common FIFO Message Over Write Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfmowsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfmowsts`] module*/
#[doc(alias = "CFDCFMOWSTS")]
pub type Cfdcfmowsts = crate::Reg<cfdcfmowsts::CfdcfmowstsSpec>;
///Common FIFO Message Over Write Status Register
pub mod cfdcfmowsts;
/**CFDFFFSTS (r) register accessor: FIFO FDC Full Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdfffsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdfffsts`] module*/
#[doc(alias = "CFDFFFSTS")]
pub type Cfdfffsts = crate::Reg<cfdfffsts::CfdfffstsSpec>;
///FIFO FDC Full Status Register
pub mod cfdfffsts;
/**CFDTMC (rw) register accessor: TX Message Buffer Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmc`] module*/
#[doc(alias = "CFDTMC")]
pub type Cfdtmc = crate::Reg<cfdtmc::CfdtmcSpec>;
///TX Message Buffer Control Registers %s
pub mod cfdtmc;
pub use Cfdtmc as Cfdtmc32;
pub use cfdtmc as cfdtmc32;
pub use Cfdtmc as Cfdtmc64;
pub use cfdtmc as cfdtmc64;
pub use Cfdtmc as Cfdtmc96;
pub use cfdtmc as cfdtmc96;
/**CFDTMSTS (rw) register accessor: TX Message Buffer Status Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmsts`] module*/
#[doc(alias = "CFDTMSTS")]
pub type Cfdtmsts = crate::Reg<cfdtmsts::CfdtmstsSpec>;
///TX Message Buffer Status Registers %s
pub mod cfdtmsts;
pub use Cfdtmsts as Cfdtmsts32;
pub use cfdtmsts as cfdtmsts32;
pub use Cfdtmsts as Cfdtmsts64;
pub use cfdtmsts as cfdtmsts64;
pub use Cfdtmsts as Cfdtmsts96;
pub use cfdtmsts as cfdtmsts96;
/**CFDTMTRSTS (r) register accessor: TX Message Buffer Transmission Request Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmtrsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmtrsts`] module*/
#[doc(alias = "CFDTMTRSTS")]
pub type Cfdtmtrsts = crate::Reg<cfdtmtrsts::CfdtmtrstsSpec>;
///TX Message Buffer Transmission Request Status Register %s
pub mod cfdtmtrsts;
/**CFDTMTARSTS (r) register accessor: TX Message Buffer Transmission Abort Request Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmtarsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmtarsts`] module*/
#[doc(alias = "CFDTMTARSTS")]
pub type Cfdtmtarsts = crate::Reg<cfdtmtarsts::CfdtmtarstsSpec>;
///TX Message Buffer Transmission Abort Request Status Register %s
pub mod cfdtmtarsts;
/**CFDTMTCSTS (r) register accessor: TX Message Buffer Transmission Completion Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmtcsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmtcsts`] module*/
#[doc(alias = "CFDTMTCSTS")]
pub type Cfdtmtcsts = crate::Reg<cfdtmtcsts::CfdtmtcstsSpec>;
///TX Message Buffer Transmission Completion Status Register %s
pub mod cfdtmtcsts;
/**CFDTMTASTS (r) register accessor: TX Message Buffer Transmission Abort Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmtasts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmtasts`] module*/
#[doc(alias = "CFDTMTASTS")]
pub type Cfdtmtasts = crate::Reg<cfdtmtasts::CfdtmtastsSpec>;
///TX Message Buffer Transmission Abort Status Register %s
pub mod cfdtmtasts;
/**CFDTMIEC (rw) register accessor: TX Message Buffer Interrupt Enable Configuration Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmiec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmiec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmiec`] module*/
#[doc(alias = "CFDTMIEC")]
pub type Cfdtmiec = crate::Reg<cfdtmiec::CfdtmiecSpec>;
///TX Message Buffer Interrupt Enable Configuration Register %s
pub mod cfdtmiec;
/**CFDTXQCC0 (rw) register accessor: TX Queue Configuration/Control Registers 0%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqcc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqcc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqcc0`] module*/
#[doc(alias = "CFDTXQCC0")]
pub type Cfdtxqcc0 = crate::Reg<cfdtxqcc0::Cfdtxqcc0Spec>;
///TX Queue Configuration/Control Registers 0%s
pub mod cfdtxqcc0;
/**CFDTXQSTS0 (rw) register accessor: TX Queue Status Registers 0%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqsts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqsts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqsts0`] module*/
#[doc(alias = "CFDTXQSTS0")]
pub type Cfdtxqsts0 = crate::Reg<cfdtxqsts0::Cfdtxqsts0Spec>;
///TX Queue Status Registers 0%s
pub mod cfdtxqsts0;
/**CFDTXQPCTR0 (rw) register accessor: TX Queue Pointer Control Registers 0%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqpctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqpctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqpctr0`] module*/
#[doc(alias = "CFDTXQPCTR0")]
pub type Cfdtxqpctr0 = crate::Reg<cfdtxqpctr0::Cfdtxqpctr0Spec>;
///TX Queue Pointer Control Registers 0%s
pub mod cfdtxqpctr0;
/**CFDTXQCC1 (rw) register accessor: TX Queue Configuration/Control Registers 1%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqcc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqcc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqcc1`] module*/
#[doc(alias = "CFDTXQCC1")]
pub type Cfdtxqcc1 = crate::Reg<cfdtxqcc1::Cfdtxqcc1Spec>;
///TX Queue Configuration/Control Registers 1%s
pub mod cfdtxqcc1;
/**CFDTXQSTS1 (rw) register accessor: TX Queue Status Registers 1%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqsts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqsts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqsts1`] module*/
#[doc(alias = "CFDTXQSTS1")]
pub type Cfdtxqsts1 = crate::Reg<cfdtxqsts1::Cfdtxqsts1Spec>;
///TX Queue Status Registers 1%s
pub mod cfdtxqsts1;
/**CFDTXQPCTR1 (rw) register accessor: TX Queue Pointer Control Registers 1%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqpctr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqpctr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqpctr1`] module*/
#[doc(alias = "CFDTXQPCTR1")]
pub type Cfdtxqpctr1 = crate::Reg<cfdtxqpctr1::Cfdtxqpctr1Spec>;
///TX Queue Pointer Control Registers 1%s
pub mod cfdtxqpctr1;
/**CFDTXQCC2 (rw) register accessor: TX Queue Configuration/Control Registers 2%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqcc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqcc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqcc2`] module*/
#[doc(alias = "CFDTXQCC2")]
pub type Cfdtxqcc2 = crate::Reg<cfdtxqcc2::Cfdtxqcc2Spec>;
///TX Queue Configuration/Control Registers 2%s
pub mod cfdtxqcc2;
/**CFDTXQSTS2 (rw) register accessor: TX Queue Status Registers 2%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqsts2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqsts2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqsts2`] module*/
#[doc(alias = "CFDTXQSTS2")]
pub type Cfdtxqsts2 = crate::Reg<cfdtxqsts2::Cfdtxqsts2Spec>;
///TX Queue Status Registers 2%s
pub mod cfdtxqsts2;
/**CFDTXQPCTR2 (rw) register accessor: TX Queue Pointer Control Registers 2%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqpctr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqpctr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqpctr2`] module*/
#[doc(alias = "CFDTXQPCTR2")]
pub type Cfdtxqpctr2 = crate::Reg<cfdtxqpctr2::Cfdtxqpctr2Spec>;
///TX Queue Pointer Control Registers 2%s
pub mod cfdtxqpctr2;
/**CFDTXQCC3 (rw) register accessor: TX Queue Configuration/Control Registers 3%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqcc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqcc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqcc3`] module*/
#[doc(alias = "CFDTXQCC3")]
pub type Cfdtxqcc3 = crate::Reg<cfdtxqcc3::Cfdtxqcc3Spec>;
///TX Queue Configuration/Control Registers 3%s
pub mod cfdtxqcc3;
/**CFDTXQSTS3 (rw) register accessor: TX Queue Status Registers 3%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqsts3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqsts3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqsts3`] module*/
#[doc(alias = "CFDTXQSTS3")]
pub type Cfdtxqsts3 = crate::Reg<cfdtxqsts3::Cfdtxqsts3Spec>;
///TX Queue Status Registers 3%s
pub mod cfdtxqsts3;
/**CFDTXQPCTR3 (rw) register accessor: TX Queue Pointer Control Registers 3%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqpctr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqpctr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqpctr3`] module*/
#[doc(alias = "CFDTXQPCTR3")]
pub type Cfdtxqpctr3 = crate::Reg<cfdtxqpctr3::Cfdtxqpctr3Spec>;
///TX Queue Pointer Control Registers 3%s
pub mod cfdtxqpctr3;
/**CFDTXQESTS (r) register accessor: TX Queue Empty Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqests::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqests`] module*/
#[doc(alias = "CFDTXQESTS")]
pub type Cfdtxqests = crate::Reg<cfdtxqests::CfdtxqestsSpec>;
///TX Queue Empty Status Register
pub mod cfdtxqests;
/**CFDTXQFISTS (rw) register accessor: TX Queue Full Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqfists::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqfists::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqfists`] module*/
#[doc(alias = "CFDTXQFISTS")]
pub type Cfdtxqfists = crate::Reg<cfdtxqfists::CfdtxqfistsSpec>;
///TX Queue Full Interrupt Status Register
pub mod cfdtxqfists;
/**CFDTXQMSTS (rw) register accessor: TX Queue Message Lost Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqmsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqmsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqmsts`] module*/
#[doc(alias = "CFDTXQMSTS")]
pub type Cfdtxqmsts = crate::Reg<cfdtxqmsts::CfdtxqmstsSpec>;
///TX Queue Message Lost Status Register
pub mod cfdtxqmsts;
/**CFDTXQISTS (rw) register accessor: TX Queue Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqists::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqists::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqists`] module*/
#[doc(alias = "CFDTXQISTS")]
pub type Cfdtxqists = crate::Reg<cfdtxqists::CfdtxqistsSpec>;
///TX Queue Interrupt Status Register
pub mod cfdtxqists;
/**CFDTXQOFTISTS (rw) register accessor: TX Queue One Frame TX Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqoftists::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqoftists::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqoftists`] module*/
#[doc(alias = "CFDTXQOFTISTS")]
pub type Cfdtxqoftists = crate::Reg<cfdtxqoftists::CfdtxqoftistsSpec>;
///TX Queue One Frame TX Interrupt Status Register
pub mod cfdtxqoftists;
/**CFDTXQOFRISTS (rw) register accessor: TX Queue One Frame RX Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqofrists::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqofrists::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqofrists`] module*/
#[doc(alias = "CFDTXQOFRISTS")]
pub type Cfdtxqofrists = crate::Reg<cfdtxqofrists::CfdtxqofristsSpec>;
///TX Queue One Frame RX Interrupt Status Register
pub mod cfdtxqofrists;
/**CFDTXQFSTS (r) register accessor: TX Queue Full Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtxqfsts`] module*/
#[doc(alias = "CFDTXQFSTS")]
pub type Cfdtxqfsts = crate::Reg<cfdtxqfsts::CfdtxqfstsSpec>;
///TX Queue Full Status Register
pub mod cfdtxqfsts;
/**CFDTHLCC (rw) register accessor: TX History List Configuration/Control Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdthlcc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdthlcc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdthlcc`] module*/
#[doc(alias = "CFDTHLCC")]
pub type Cfdthlcc = crate::Reg<cfdthlcc::CfdthlccSpec>;
///TX History List Configuration/Control Register %s
pub mod cfdthlcc;
/**CFDTHLSTS (rw) register accessor: TX History List Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdthlsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdthlsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdthlsts`] module*/
#[doc(alias = "CFDTHLSTS")]
pub type Cfdthlsts = crate::Reg<cfdthlsts::CfdthlstsSpec>;
///TX History List Status Register %s
pub mod cfdthlsts;
/**CFDTHLPCTR (rw) register accessor: TX History List Pointer Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdthlpctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdthlpctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdthlpctr`] module*/
#[doc(alias = "CFDTHLPCTR")]
pub type Cfdthlpctr = crate::Reg<cfdthlpctr::CfdthlpctrSpec>;
///TX History List Pointer Control Registers %s
pub mod cfdthlpctr;
/**CFDGTINTSTS0 (rw) register accessor: Global TX Interrupt Status Register 0

You can [`read`](crate::Reg::read) this register and get [`cfdgtintsts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgtintsts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgtintsts0`] module*/
#[doc(alias = "CFDGTINTSTS0")]
pub type Cfdgtintsts0 = crate::Reg<cfdgtintsts0::Cfdgtintsts0Spec>;
///Global TX Interrupt Status Register 0
pub mod cfdgtintsts0;
/**CFDGTSTCFG (rw) register accessor: Global Test Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdgtstcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgtstcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgtstcfg`] module*/
#[doc(alias = "CFDGTSTCFG")]
pub type Cfdgtstcfg = crate::Reg<cfdgtstcfg::CfdgtstcfgSpec>;
///Global Test Configuration Register
pub mod cfdgtstcfg;
/**CFDGTSTCTR (rw) register accessor: Global Test Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdgtstctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgtstctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgtstctr`] module*/
#[doc(alias = "CFDGTSTCTR")]
pub type Cfdgtstctr = crate::Reg<cfdgtstctr::CfdgtstctrSpec>;
///Global Test Control Register
pub mod cfdgtstctr;
/**CFDGFDCFG (rw) register accessor: Global FD Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdgfdcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgfdcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgfdcfg`] module*/
#[doc(alias = "CFDGFDCFG")]
pub type Cfdgfdcfg = crate::Reg<cfdgfdcfg::CfdgfdcfgSpec>;
///Global FD Configuration Register
pub mod cfdgfdcfg;
/**CFDGLOCKK (w) register accessor: Global Lock Key Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdglockk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdglockk`] module*/
#[doc(alias = "CFDGLOCKK")]
pub type Cfdglockk = crate::Reg<cfdglockk::CfdglockkSpec>;
///Global Lock Key Register
pub mod cfdglockk;
/**CFDCDTCT (rw) register accessor: DMA Transfer Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdcdtct::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcdtct::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcdtct`] module*/
#[doc(alias = "CFDCDTCT")]
pub type Cfdcdtct = crate::Reg<cfdcdtct::CfdcdtctSpec>;
///DMA Transfer Control Register
pub mod cfdcdtct;
/**CFDCDTSTS (r) register accessor: DMA Transfer Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcdtsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcdtsts`] module*/
#[doc(alias = "CFDCDTSTS")]
pub type Cfdcdtsts = crate::Reg<cfdcdtsts::CfdcdtstsSpec>;
///DMA Transfer Status Register
pub mod cfdcdtsts;
/**CFDCDTTCT (rw) register accessor: DMA TX Transfer Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdcdttct::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcdttct::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcdttct`] module*/
#[doc(alias = "CFDCDTTCT")]
pub type Cfdcdttct = crate::Reg<cfdcdttct::CfdcdttctSpec>;
///DMA TX Transfer Control Register
pub mod cfdcdttct;
/**CFDCDTTSTS (rw) register accessor: DMA TX Transfer Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcdttsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcdttsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcdttsts`] module*/
#[doc(alias = "CFDCDTTSTS")]
pub type Cfdcdttsts = crate::Reg<cfdcdttsts::CfdcdttstsSpec>;
///DMA TX Transfer Status Register
pub mod cfdcdttsts;
/**CFDGRINTSTS (rw) register accessor: Global RX Interrupt Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdgrintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgrintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgrintsts`] module*/
#[doc(alias = "CFDGRINTSTS")]
pub type Cfdgrintsts = crate::Reg<cfdgrintsts::CfdgrintstsSpec>;
///Global RX Interrupt Status Register %s
pub mod cfdgrintsts;
/**CFDGRSTC (rw) register accessor: Global SW reset Register

You can [`read`](crate::Reg::read) this register and get [`cfdgrstc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgrstc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgrstc`] module*/
#[doc(alias = "CFDGRSTC")]
pub type Cfdgrstc = crate::Reg<cfdgrstc::CfdgrstcSpec>;
///Global SW reset Register
pub mod cfdgrstc;
/**CFDCDCFG (rw) register accessor: Channel %s Data Bitrate Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdcdcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcdcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcdcfg`] module*/
#[doc(alias = "CFDCDCFG")]
pub type Cfdcdcfg = crate::Reg<cfdcdcfg::CfdcdcfgSpec>;
///Channel %s Data Bitrate Configuration Register
pub mod cfdcdcfg;
/**CFDCFDCFG (rw) register accessor: Channel %s CAN-FD Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfdcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdcfg`] module*/
#[doc(alias = "CFDCFDCFG")]
pub type Cfdcfdcfg = crate::Reg<cfdcfdcfg::CfdcfdcfgSpec>;
///Channel %s CAN-FD Configuration Register
pub mod cfdcfdcfg;
/**CFDCFDCTR (rw) register accessor: Channel %s CANFD Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfdctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdctr`] module*/
#[doc(alias = "CFDCFDCTR")]
pub type Cfdcfdctr = crate::Reg<cfdcfdctr::CfdcfdctrSpec>;
///Channel %s CANFD Control Register
pub mod cfdcfdctr;
/**CFDCFDSTS (rw) register accessor: Channel %s CANFD Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfdsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdsts`] module*/
#[doc(alias = "CFDCFDSTS")]
pub type Cfdcfdsts = crate::Reg<cfdcfdsts::CfdcfdstsSpec>;
///Channel %s CANFD Status Register
pub mod cfdcfdsts;
/**CFDCFDCRC (rw) register accessor: Channel %s CANFD CRC Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfdcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdcrc`] module*/
#[doc(alias = "CFDCFDCRC")]
pub type Cfdcfdcrc = crate::Reg<cfdcfdcrc::CfdcfdcrcSpec>;
///Channel %s CANFD CRC Register
pub mod cfdcfdcrc;
/**CFDCBLCT (rw) register accessor: Channel %s Bus Load Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdcblct::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcblct::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcblct`] module*/
#[doc(alias = "CFDCBLCT")]
pub type Cfdcblct = crate::Reg<cfdcblct::CfdcblctSpec>;
///Channel %s Bus Load Control Register
pub mod cfdcblct;
/**CFDCBLSTS (rw) register accessor: Channel %s Bus Load Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcblsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcblsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcblsts`] module*/
#[doc(alias = "CFDCBLSTS")]
pub type Cfdcblsts = crate::Reg<cfdcblsts::CfdcblstsSpec>;
///Channel %s Bus Load Status Register
pub mod cfdcblsts;
/**CFDGAFLID (rw) register accessor: Global Acceptance Filter List ID Registers

You can [`read`](crate::Reg::read) this register and get [`cfdgaflid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgaflid`] module*/
#[doc(alias = "CFDGAFLID")]
pub type Cfdgaflid = crate::Reg<cfdgaflid::CfdgaflidSpec>;
///Global Acceptance Filter List ID Registers
pub mod cfdgaflid;
/**CFDGAFLM (rw) register accessor: Global Acceptance Filter List Mask Registers

You can [`read`](crate::Reg::read) this register and get [`cfdgaflm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgaflm`] module*/
#[doc(alias = "CFDGAFLM")]
pub type Cfdgaflm = crate::Reg<cfdgaflm::CfdgaflmSpec>;
///Global Acceptance Filter List Mask Registers
pub mod cfdgaflm;
/**CFDGAFLP0 (rw) register accessor: Global Acceptance Filter List Pointer 0 Registers

You can [`read`](crate::Reg::read) this register and get [`cfdgaflp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgaflp0`] module*/
#[doc(alias = "CFDGAFLP0")]
pub type Cfdgaflp0 = crate::Reg<cfdgaflp0::Cfdgaflp0Spec>;
///Global Acceptance Filter List Pointer 0 Registers
pub mod cfdgaflp0;
/**CFDGAFLP1 (rw) register accessor: Global Acceptance Filter List Pointer 1 Registers

You can [`read`](crate::Reg::read) this register and get [`cfdgaflp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdgaflp1`] module*/
#[doc(alias = "CFDGAFLP1")]
pub type Cfdgaflp1 = crate::Reg<cfdgaflp1::Cfdgaflp1Spec>;
///Global Acceptance Filter List Pointer 1 Registers
pub mod cfdgaflp1;
/**CFDRMID_0 (r) register accessor: RX Message Buffer ID Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmid_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmid_0`] module*/
#[doc(alias = "CFDRMID_0")]
pub type Cfdrmid0 = crate::Reg<cfdrmid_0::Cfdrmid0Spec>;
///RX Message Buffer ID Register %s Channel 0
pub mod cfdrmid_0;
/**CFDRMPTR_0 (r) register accessor: RX Message Buffer Pointer Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmptr_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmptr_0`] module*/
#[doc(alias = "CFDRMPTR_0")]
pub type Cfdrmptr0 = crate::Reg<cfdrmptr_0::Cfdrmptr0Spec>;
///RX Message Buffer Pointer Register %s Channel 0
pub mod cfdrmptr_0;
/**CFDRMFDSTS_0 (r) register accessor: RX Message Buffer CAN-FD Status Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmfdsts_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmfdsts_0`] module*/
#[doc(alias = "CFDRMFDSTS_0")]
pub type Cfdrmfdsts0 = crate::Reg<cfdrmfdsts_0::Cfdrmfdsts0Spec>;
///RX Message Buffer CAN-FD Status Register %s Channel 0
pub mod cfdrmfdsts_0;
/**CFDRMDF0__0 (r) register accessor: RX Message Buffer Data Field 0 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf0__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf0__0`] module*/
#[doc(alias = "CFDRMDF0__0")]
pub type Cfdrmdf0_0 = crate::Reg<cfdrmdf0__0::Cfdrmdf0_0Spec>;
///RX Message Buffer Data Field 0 Register %s Channel 0
pub mod cfdrmdf0__0;
/**CFDRMDF1__0 (r) register accessor: RX Message Buffer Data Field 1 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf1__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf1__0`] module*/
#[doc(alias = "CFDRMDF1__0")]
pub type Cfdrmdf1_0 = crate::Reg<cfdrmdf1__0::Cfdrmdf1_0Spec>;
///RX Message Buffer Data Field 1 Register %s Channel 0
pub mod cfdrmdf1__0;
/**CFDRMDF2__0 (r) register accessor: RX Message Buffer Data Field 2 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf2__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf2__0`] module*/
#[doc(alias = "CFDRMDF2__0")]
pub type Cfdrmdf2_0 = crate::Reg<cfdrmdf2__0::Cfdrmdf2_0Spec>;
///RX Message Buffer Data Field 2 Register %s Channel 0
pub mod cfdrmdf2__0;
/**CFDRMDF3__0 (r) register accessor: RX Message Buffer Data Field 3 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf3__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf3__0`] module*/
#[doc(alias = "CFDRMDF3__0")]
pub type Cfdrmdf3_0 = crate::Reg<cfdrmdf3__0::Cfdrmdf3_0Spec>;
///RX Message Buffer Data Field 3 Register %s Channel 0
pub mod cfdrmdf3__0;
/**CFDRMDF4__0 (r) register accessor: RX Message Buffer Data Field 4 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf4__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf4__0`] module*/
#[doc(alias = "CFDRMDF4__0")]
pub type Cfdrmdf4_0 = crate::Reg<cfdrmdf4__0::Cfdrmdf4_0Spec>;
///RX Message Buffer Data Field 4 Register %s Channel 0
pub mod cfdrmdf4__0;
/**CFDRMDF5__0 (r) register accessor: RX Message Buffer Data Field 5 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf5__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf5__0`] module*/
#[doc(alias = "CFDRMDF5__0")]
pub type Cfdrmdf5_0 = crate::Reg<cfdrmdf5__0::Cfdrmdf5_0Spec>;
///RX Message Buffer Data Field 5 Register %s Channel 0
pub mod cfdrmdf5__0;
/**CFDRMDF6__0 (r) register accessor: RX Message Buffer Data Field 6 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf6__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf6__0`] module*/
#[doc(alias = "CFDRMDF6__0")]
pub type Cfdrmdf6_0 = crate::Reg<cfdrmdf6__0::Cfdrmdf6_0Spec>;
///RX Message Buffer Data Field 6 Register %s Channel 0
pub mod cfdrmdf6__0;
/**CFDRMDF7__0 (r) register accessor: RX Message Buffer Data Field 7 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf7__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf7__0`] module*/
#[doc(alias = "CFDRMDF7__0")]
pub type Cfdrmdf7_0 = crate::Reg<cfdrmdf7__0::Cfdrmdf7_0Spec>;
///RX Message Buffer Data Field 7 Register %s Channel 0
pub mod cfdrmdf7__0;
/**CFDRMDF8__0 (r) register accessor: RX Message Buffer Data Field 8 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf8__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf8__0`] module*/
#[doc(alias = "CFDRMDF8__0")]
pub type Cfdrmdf8_0 = crate::Reg<cfdrmdf8__0::Cfdrmdf8_0Spec>;
///RX Message Buffer Data Field 8 Register %s Channel 0
pub mod cfdrmdf8__0;
/**CFDRMDF9__0 (r) register accessor: RX Message Buffer Data Field 9 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf9__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf9__0`] module*/
#[doc(alias = "CFDRMDF9__0")]
pub type Cfdrmdf9_0 = crate::Reg<cfdrmdf9__0::Cfdrmdf9_0Spec>;
///RX Message Buffer Data Field 9 Register %s Channel 0
pub mod cfdrmdf9__0;
/**CFDRMDF10__0 (r) register accessor: RX Message Buffer Data Field 10 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf10__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf10__0`] module*/
#[doc(alias = "CFDRMDF10__0")]
pub type Cfdrmdf10_0 = crate::Reg<cfdrmdf10__0::Cfdrmdf10_0Spec>;
///RX Message Buffer Data Field 10 Register %s Channel 0
pub mod cfdrmdf10__0;
/**CFDRMDF11__0 (r) register accessor: RX Message Buffer Data Field 11 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf11__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf11__0`] module*/
#[doc(alias = "CFDRMDF11__0")]
pub type Cfdrmdf11_0 = crate::Reg<cfdrmdf11__0::Cfdrmdf11_0Spec>;
///RX Message Buffer Data Field 11 Register %s Channel 0
pub mod cfdrmdf11__0;
/**CFDRMDF12__0 (r) register accessor: RX Message Buffer Data Field 12 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf12__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf12__0`] module*/
#[doc(alias = "CFDRMDF12__0")]
pub type Cfdrmdf12_0 = crate::Reg<cfdrmdf12__0::Cfdrmdf12_0Spec>;
///RX Message Buffer Data Field 12 Register %s Channel 0
pub mod cfdrmdf12__0;
/**CFDRMDF13__0 (r) register accessor: RX Message Buffer Data Field 13 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf13__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf13__0`] module*/
#[doc(alias = "CFDRMDF13__0")]
pub type Cfdrmdf13_0 = crate::Reg<cfdrmdf13__0::Cfdrmdf13_0Spec>;
///RX Message Buffer Data Field 13 Register %s Channel 0
pub mod cfdrmdf13__0;
/**CFDRMDF14__0 (r) register accessor: RX Message Buffer Data Field 14 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf14__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf14__0`] module*/
#[doc(alias = "CFDRMDF14__0")]
pub type Cfdrmdf14_0 = crate::Reg<cfdrmdf14__0::Cfdrmdf14_0Spec>;
///RX Message Buffer Data Field 14 Register %s Channel 0
pub mod cfdrmdf14__0;
/**CFDRMDF15__0 (r) register accessor: RX Message Buffer Data Field 15 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf15__0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf15__0`] module*/
#[doc(alias = "CFDRMDF15__0")]
pub type Cfdrmdf15_0 = crate::Reg<cfdrmdf15__0::Cfdrmdf15_0Spec>;
///RX Message Buffer Data Field 15 Register %s Channel 0
pub mod cfdrmdf15__0;
/**CFDRMID_1 (r) register accessor: RX Message Buffer ID Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmid_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmid_1`] module*/
#[doc(alias = "CFDRMID_1")]
pub type Cfdrmid1 = crate::Reg<cfdrmid_1::Cfdrmid1Spec>;
///RX Message Buffer ID Register %s Channel 1
pub mod cfdrmid_1;
/**CFDRMPTR_1 (r) register accessor: RX Message Buffer Pointer Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmptr_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmptr_1`] module*/
#[doc(alias = "CFDRMPTR_1")]
pub type Cfdrmptr1 = crate::Reg<cfdrmptr_1::Cfdrmptr1Spec>;
///RX Message Buffer Pointer Register %s Channel 1
pub mod cfdrmptr_1;
/**CFDRMFDSTS_1 (r) register accessor: RX Message Buffer CAN-FD Status Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmfdsts_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmfdsts_1`] module*/
#[doc(alias = "CFDRMFDSTS_1")]
pub type Cfdrmfdsts1 = crate::Reg<cfdrmfdsts_1::Cfdrmfdsts1Spec>;
///RX Message Buffer CAN-FD Status Register %s Channel 1
pub mod cfdrmfdsts_1;
/**CFDRMDF0__1 (r) register accessor: RX Message Buffer Data Field 0 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf0__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf0__1`] module*/
#[doc(alias = "CFDRMDF0__1")]
pub type Cfdrmdf0_1 = crate::Reg<cfdrmdf0__1::Cfdrmdf0_1Spec>;
///RX Message Buffer Data Field 0 Register %s Channel 1
pub mod cfdrmdf0__1;
/**CFDRMDF1__1 (r) register accessor: RX Message Buffer Data Field 1 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf1__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf1__1`] module*/
#[doc(alias = "CFDRMDF1__1")]
pub type Cfdrmdf1_1 = crate::Reg<cfdrmdf1__1::Cfdrmdf1_1Spec>;
///RX Message Buffer Data Field 1 Register %s Channel 1
pub mod cfdrmdf1__1;
/**CFDRMDF2__1 (r) register accessor: RX Message Buffer Data Field 2 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf2__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf2__1`] module*/
#[doc(alias = "CFDRMDF2__1")]
pub type Cfdrmdf2_1 = crate::Reg<cfdrmdf2__1::Cfdrmdf2_1Spec>;
///RX Message Buffer Data Field 2 Register %s Channel 1
pub mod cfdrmdf2__1;
/**CFDRMDF3__1 (r) register accessor: RX Message Buffer Data Field 3 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf3__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf3__1`] module*/
#[doc(alias = "CFDRMDF3__1")]
pub type Cfdrmdf3_1 = crate::Reg<cfdrmdf3__1::Cfdrmdf3_1Spec>;
///RX Message Buffer Data Field 3 Register %s Channel 1
pub mod cfdrmdf3__1;
/**CFDRMDF4__1 (r) register accessor: RX Message Buffer Data Field 4 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf4__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf4__1`] module*/
#[doc(alias = "CFDRMDF4__1")]
pub type Cfdrmdf4_1 = crate::Reg<cfdrmdf4__1::Cfdrmdf4_1Spec>;
///RX Message Buffer Data Field 4 Register %s Channel 1
pub mod cfdrmdf4__1;
/**CFDRMDF5__1 (r) register accessor: RX Message Buffer Data Field 5 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf5__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf5__1`] module*/
#[doc(alias = "CFDRMDF5__1")]
pub type Cfdrmdf5_1 = crate::Reg<cfdrmdf5__1::Cfdrmdf5_1Spec>;
///RX Message Buffer Data Field 5 Register %s Channel 1
pub mod cfdrmdf5__1;
/**CFDRMDF6__1 (r) register accessor: RX Message Buffer Data Field 6 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf6__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf6__1`] module*/
#[doc(alias = "CFDRMDF6__1")]
pub type Cfdrmdf6_1 = crate::Reg<cfdrmdf6__1::Cfdrmdf6_1Spec>;
///RX Message Buffer Data Field 6 Register %s Channel 1
pub mod cfdrmdf6__1;
/**CFDRMDF7__1 (r) register accessor: RX Message Buffer Data Field 7 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf7__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf7__1`] module*/
#[doc(alias = "CFDRMDF7__1")]
pub type Cfdrmdf7_1 = crate::Reg<cfdrmdf7__1::Cfdrmdf7_1Spec>;
///RX Message Buffer Data Field 7 Register %s Channel 1
pub mod cfdrmdf7__1;
/**CFDRMDF8__1 (r) register accessor: RX Message Buffer Data Field 8 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf8__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf8__1`] module*/
#[doc(alias = "CFDRMDF8__1")]
pub type Cfdrmdf8_1 = crate::Reg<cfdrmdf8__1::Cfdrmdf8_1Spec>;
///RX Message Buffer Data Field 8 Register %s Channel 1
pub mod cfdrmdf8__1;
/**CFDRMDF9__1 (r) register accessor: RX Message Buffer Data Field 9 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf9__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf9__1`] module*/
#[doc(alias = "CFDRMDF9__1")]
pub type Cfdrmdf9_1 = crate::Reg<cfdrmdf9__1::Cfdrmdf9_1Spec>;
///RX Message Buffer Data Field 9 Register %s Channel 1
pub mod cfdrmdf9__1;
/**CFDRMDF10__1 (r) register accessor: RX Message Buffer Data Field 10 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf10__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf10__1`] module*/
#[doc(alias = "CFDRMDF10__1")]
pub type Cfdrmdf10_1 = crate::Reg<cfdrmdf10__1::Cfdrmdf10_1Spec>;
///RX Message Buffer Data Field 10 Register %s Channel 1
pub mod cfdrmdf10__1;
/**CFDRMDF11__1 (r) register accessor: RX Message Buffer Data Field 11 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf11__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf11__1`] module*/
#[doc(alias = "CFDRMDF11__1")]
pub type Cfdrmdf11_1 = crate::Reg<cfdrmdf11__1::Cfdrmdf11_1Spec>;
///RX Message Buffer Data Field 11 Register %s Channel 1
pub mod cfdrmdf11__1;
/**CFDRMDF12__1 (r) register accessor: RX Message Buffer Data Field 12 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf12__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf12__1`] module*/
#[doc(alias = "CFDRMDF12__1")]
pub type Cfdrmdf12_1 = crate::Reg<cfdrmdf12__1::Cfdrmdf12_1Spec>;
///RX Message Buffer Data Field 12 Register %s Channel 1
pub mod cfdrmdf12__1;
/**CFDRMDF13__1 (r) register accessor: RX Message Buffer Data Field 13 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf13__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf13__1`] module*/
#[doc(alias = "CFDRMDF13__1")]
pub type Cfdrmdf13_1 = crate::Reg<cfdrmdf13__1::Cfdrmdf13_1Spec>;
///RX Message Buffer Data Field 13 Register %s Channel 1
pub mod cfdrmdf13__1;
/**CFDRMDF14__1 (r) register accessor: RX Message Buffer Data Field 14 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf14__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf14__1`] module*/
#[doc(alias = "CFDRMDF14__1")]
pub type Cfdrmdf14_1 = crate::Reg<cfdrmdf14__1::Cfdrmdf14_1Spec>;
///RX Message Buffer Data Field 14 Register %s Channel 1
pub mod cfdrmdf14__1;
/**CFDRMDF15__1 (r) register accessor: RX Message Buffer Data Field 15 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf15__1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrmdf15__1`] module*/
#[doc(alias = "CFDRMDF15__1")]
pub type Cfdrmdf15_1 = crate::Reg<cfdrmdf15__1::Cfdrmdf15_1Spec>;
///RX Message Buffer Data Field 15 Register %s Channel 1
pub mod cfdrmdf15__1;
/**CFDRFID (r) register accessor: RX FIFO Access ID Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfid`] module*/
#[doc(alias = "CFDRFID")]
pub type Cfdrfid = crate::Reg<cfdrfid::CfdrfidSpec>;
///RX FIFO Access ID Register %s
pub mod cfdrfid;
/**CFDRFPTR (r) register accessor: RX FIFO Access Pointer Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfptr`] module*/
#[doc(alias = "CFDRFPTR")]
pub type Cfdrfptr = crate::Reg<cfdrfptr::CfdrfptrSpec>;
///RX FIFO Access Pointer Register %s
pub mod cfdrfptr;
/**CFDRFFDSTS (r) register accessor: RX FIFO Access CAN-FD Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrffdsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrffdsts`] module*/
#[doc(alias = "CFDRFFDSTS")]
pub type Cfdrffdsts = crate::Reg<cfdrffdsts::CfdrffdstsSpec>;
///RX FIFO Access CAN-FD Status Register %s
pub mod cfdrffdsts;
/**CFDRFDF0 (r) register accessor: RX FIFO Access Data Field 0 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf0`] module*/
#[doc(alias = "CFDRFDF0")]
pub type Cfdrfdf0 = crate::Reg<cfdrfdf0::Cfdrfdf0Spec>;
///RX FIFO Access Data Field 0 Register %s
pub mod cfdrfdf0;
/**CFDRFDF1 (r) register accessor: RX FIFO Access Data Field 1 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf1`] module*/
#[doc(alias = "CFDRFDF1")]
pub type Cfdrfdf1 = crate::Reg<cfdrfdf1::Cfdrfdf1Spec>;
///RX FIFO Access Data Field 1 Register %s
pub mod cfdrfdf1;
/**CFDRFDF2 (r) register accessor: RX FIFO Access Data Field 2 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf2`] module*/
#[doc(alias = "CFDRFDF2")]
pub type Cfdrfdf2 = crate::Reg<cfdrfdf2::Cfdrfdf2Spec>;
///RX FIFO Access Data Field 2 Register %s
pub mod cfdrfdf2;
/**CFDRFDF3 (r) register accessor: RX FIFO Access Data Field 3 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf3`] module*/
#[doc(alias = "CFDRFDF3")]
pub type Cfdrfdf3 = crate::Reg<cfdrfdf3::Cfdrfdf3Spec>;
///RX FIFO Access Data Field 3 Register %s
pub mod cfdrfdf3;
/**CFDRFDF4 (r) register accessor: RX FIFO Access Data Field 4 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf4`] module*/
#[doc(alias = "CFDRFDF4")]
pub type Cfdrfdf4 = crate::Reg<cfdrfdf4::Cfdrfdf4Spec>;
///RX FIFO Access Data Field 4 Register %s
pub mod cfdrfdf4;
/**CFDRFDF5 (r) register accessor: RX FIFO Access Data Field 5 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf5`] module*/
#[doc(alias = "CFDRFDF5")]
pub type Cfdrfdf5 = crate::Reg<cfdrfdf5::Cfdrfdf5Spec>;
///RX FIFO Access Data Field 5 Register %s
pub mod cfdrfdf5;
/**CFDRFDF6 (r) register accessor: RX FIFO Access Data Field 6 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf6`] module*/
#[doc(alias = "CFDRFDF6")]
pub type Cfdrfdf6 = crate::Reg<cfdrfdf6::Cfdrfdf6Spec>;
///RX FIFO Access Data Field 6 Register %s
pub mod cfdrfdf6;
/**CFDRFDF7 (r) register accessor: RX FIFO Access Data Field 7 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf7`] module*/
#[doc(alias = "CFDRFDF7")]
pub type Cfdrfdf7 = crate::Reg<cfdrfdf7::Cfdrfdf7Spec>;
///RX FIFO Access Data Field 7 Register %s
pub mod cfdrfdf7;
/**CFDRFDF8 (r) register accessor: RX FIFO Access Data Field 8 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf8`] module*/
#[doc(alias = "CFDRFDF8")]
pub type Cfdrfdf8 = crate::Reg<cfdrfdf8::Cfdrfdf8Spec>;
///RX FIFO Access Data Field 8 Register %s
pub mod cfdrfdf8;
/**CFDRFDF9 (r) register accessor: RX FIFO Access Data Field 9 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf9`] module*/
#[doc(alias = "CFDRFDF9")]
pub type Cfdrfdf9 = crate::Reg<cfdrfdf9::Cfdrfdf9Spec>;
///RX FIFO Access Data Field 9 Register %s
pub mod cfdrfdf9;
/**CFDRFDF10 (r) register accessor: RX FIFO Access Data Field 10 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf10`] module*/
#[doc(alias = "CFDRFDF10")]
pub type Cfdrfdf10 = crate::Reg<cfdrfdf10::Cfdrfdf10Spec>;
///RX FIFO Access Data Field 10 Register %s
pub mod cfdrfdf10;
/**CFDRFDF11 (r) register accessor: RX FIFO Access Data Field 11 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf11`] module*/
#[doc(alias = "CFDRFDF11")]
pub type Cfdrfdf11 = crate::Reg<cfdrfdf11::Cfdrfdf11Spec>;
///RX FIFO Access Data Field 11 Register %s
pub mod cfdrfdf11;
/**CFDRFDF12 (r) register accessor: RX FIFO Access Data Field 12 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf12`] module*/
#[doc(alias = "CFDRFDF12")]
pub type Cfdrfdf12 = crate::Reg<cfdrfdf12::Cfdrfdf12Spec>;
///RX FIFO Access Data Field 12 Register %s
pub mod cfdrfdf12;
/**CFDRFDF13 (r) register accessor: RX FIFO Access Data Field 13 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf13`] module*/
#[doc(alias = "CFDRFDF13")]
pub type Cfdrfdf13 = crate::Reg<cfdrfdf13::Cfdrfdf13Spec>;
///RX FIFO Access Data Field 13 Register %s
pub mod cfdrfdf13;
/**CFDRFDF14 (r) register accessor: RX FIFO Access Data Field 14 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf14`] module*/
#[doc(alias = "CFDRFDF14")]
pub type Cfdrfdf14 = crate::Reg<cfdrfdf14::Cfdrfdf14Spec>;
///RX FIFO Access Data Field 14 Register %s
pub mod cfdrfdf14;
/**CFDRFDF15 (r) register accessor: RX FIFO Access Data Field 15 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrfdf15`] module*/
#[doc(alias = "CFDRFDF15")]
pub type Cfdrfdf15 = crate::Reg<cfdrfdf15::Cfdrfdf15Spec>;
///RX FIFO Access Data Field 15 Register %s
pub mod cfdrfdf15;
/**CFDCFID_0 (rw) register accessor: Common FIFO Access ID Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfid_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfid_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfid_0`] module*/
#[doc(alias = "CFDCFID_0")]
pub type Cfdcfid0 = crate::Reg<cfdcfid_0::Cfdcfid0Spec>;
///Common FIFO Access ID Register %s Channel 0
pub mod cfdcfid_0;
/**CFDCFPTR_0 (rw) register accessor: Common FIFO Access Pointer Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfptr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfptr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfptr_0`] module*/
#[doc(alias = "CFDCFPTR_0")]
pub type Cfdcfptr0 = crate::Reg<cfdcfptr_0::Cfdcfptr0Spec>;
///Common FIFO Access Pointer Register %s Channel 0
pub mod cfdcfptr_0;
/**CFDCFFDCSTS_0 (rw) register accessor: Common FIFO Access CAN-FD Control/Status Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcffdcsts_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcffdcsts_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcffdcsts_0`] module*/
#[doc(alias = "CFDCFFDCSTS_0")]
pub type Cfdcffdcsts0 = crate::Reg<cfdcffdcsts_0::Cfdcffdcsts0Spec>;
///Common FIFO Access CAN-FD Control/Status Register %s Channel 0
pub mod cfdcffdcsts_0;
/**CFDCFDF0_0 (rw) register accessor: Common FIFO Access Data Field 0 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf0_0`] module*/
#[doc(alias = "CFDCFDF0_0")]
pub type Cfdcfdf0_0 = crate::Reg<cfdcfdf0_0::Cfdcfdf0_0Spec>;
///Common FIFO Access Data Field 0 Register %s Channel 0
pub mod cfdcfdf0_0;
/**CFDCFDF1_0 (rw) register accessor: Common FIFO Access Data Field 1 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf1_0`] module*/
#[doc(alias = "CFDCFDF1_0")]
pub type Cfdcfdf1_0 = crate::Reg<cfdcfdf1_0::Cfdcfdf1_0Spec>;
///Common FIFO Access Data Field 1 Register %s Channel 0
pub mod cfdcfdf1_0;
/**CFDCFDF2_0 (rw) register accessor: Common FIFO Access Data Field 2 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf2_0`] module*/
#[doc(alias = "CFDCFDF2_0")]
pub type Cfdcfdf2_0 = crate::Reg<cfdcfdf2_0::Cfdcfdf2_0Spec>;
///Common FIFO Access Data Field 2 Register %s Channel 0
pub mod cfdcfdf2_0;
/**CFDCFDF3_0 (rw) register accessor: Common FIFO Access Data Field 3 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf3_0`] module*/
#[doc(alias = "CFDCFDF3_0")]
pub type Cfdcfdf3_0 = crate::Reg<cfdcfdf3_0::Cfdcfdf3_0Spec>;
///Common FIFO Access Data Field 3 Register %s Channel 0
pub mod cfdcfdf3_0;
/**CFDCFDF4_0 (rw) register accessor: Common FIFO Access Data Field 4 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf4_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf4_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf4_0`] module*/
#[doc(alias = "CFDCFDF4_0")]
pub type Cfdcfdf4_0 = crate::Reg<cfdcfdf4_0::Cfdcfdf4_0Spec>;
///Common FIFO Access Data Field 4 Register %s Channel 0
pub mod cfdcfdf4_0;
/**CFDCFDF5_0 (rw) register accessor: Common FIFO Access Data Field 5 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf5_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf5_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf5_0`] module*/
#[doc(alias = "CFDCFDF5_0")]
pub type Cfdcfdf5_0 = crate::Reg<cfdcfdf5_0::Cfdcfdf5_0Spec>;
///Common FIFO Access Data Field 5 Register %s Channel 0
pub mod cfdcfdf5_0;
/**CFDCFDF6_0 (rw) register accessor: Common FIFO Access Data Field 6 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf6_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf6_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf6_0`] module*/
#[doc(alias = "CFDCFDF6_0")]
pub type Cfdcfdf6_0 = crate::Reg<cfdcfdf6_0::Cfdcfdf6_0Spec>;
///Common FIFO Access Data Field 6 Register %s Channel 0
pub mod cfdcfdf6_0;
/**CFDCFDF7_0 (rw) register accessor: Common FIFO Access Data Field 7 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf7_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf7_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf7_0`] module*/
#[doc(alias = "CFDCFDF7_0")]
pub type Cfdcfdf7_0 = crate::Reg<cfdcfdf7_0::Cfdcfdf7_0Spec>;
///Common FIFO Access Data Field 7 Register %s Channel 0
pub mod cfdcfdf7_0;
/**CFDCFDF8_0 (rw) register accessor: Common FIFO Access Data Field 8 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf8_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf8_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf8_0`] module*/
#[doc(alias = "CFDCFDF8_0")]
pub type Cfdcfdf8_0 = crate::Reg<cfdcfdf8_0::Cfdcfdf8_0Spec>;
///Common FIFO Access Data Field 8 Register %s Channel 0
pub mod cfdcfdf8_0;
/**CFDCFDF9_0 (rw) register accessor: Common FIFO Access Data Field 9 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf9_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf9_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf9_0`] module*/
#[doc(alias = "CFDCFDF9_0")]
pub type Cfdcfdf9_0 = crate::Reg<cfdcfdf9_0::Cfdcfdf9_0Spec>;
///Common FIFO Access Data Field 9 Register %s Channel 0
pub mod cfdcfdf9_0;
/**CFDCFDF10_0 (rw) register accessor: Common FIFO Access Data Field 10 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf10_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf10_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf10_0`] module*/
#[doc(alias = "CFDCFDF10_0")]
pub type Cfdcfdf10_0 = crate::Reg<cfdcfdf10_0::Cfdcfdf10_0Spec>;
///Common FIFO Access Data Field 10 Register %s Channel 0
pub mod cfdcfdf10_0;
/**CFDCFDF11_0 (rw) register accessor: Common FIFO Access Data Field 11 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf11_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf11_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf11_0`] module*/
#[doc(alias = "CFDCFDF11_0")]
pub type Cfdcfdf11_0 = crate::Reg<cfdcfdf11_0::Cfdcfdf11_0Spec>;
///Common FIFO Access Data Field 11 Register %s Channel 0
pub mod cfdcfdf11_0;
/**CFDCFDF12_0 (rw) register accessor: Common FIFO Access Data Field 12 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf12_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf12_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf12_0`] module*/
#[doc(alias = "CFDCFDF12_0")]
pub type Cfdcfdf12_0 = crate::Reg<cfdcfdf12_0::Cfdcfdf12_0Spec>;
///Common FIFO Access Data Field 12 Register %s Channel 0
pub mod cfdcfdf12_0;
/**CFDCFDF13_0 (rw) register accessor: Common FIFO Access Data Field 13 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf13_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf13_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf13_0`] module*/
#[doc(alias = "CFDCFDF13_0")]
pub type Cfdcfdf13_0 = crate::Reg<cfdcfdf13_0::Cfdcfdf13_0Spec>;
///Common FIFO Access Data Field 13 Register %s Channel 0
pub mod cfdcfdf13_0;
/**CFDCFDF14_0 (rw) register accessor: Common FIFO Access Data Field 14 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf14_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf14_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf14_0`] module*/
#[doc(alias = "CFDCFDF14_0")]
pub type Cfdcfdf14_0 = crate::Reg<cfdcfdf14_0::Cfdcfdf14_0Spec>;
///Common FIFO Access Data Field 14 Register %s Channel 0
pub mod cfdcfdf14_0;
/**CFDCFDF15_0 (rw) register accessor: Common FIFO Access Data Field 15 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf15_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf15_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf15_0`] module*/
#[doc(alias = "CFDCFDF15_0")]
pub type Cfdcfdf15_0 = crate::Reg<cfdcfdf15_0::Cfdcfdf15_0Spec>;
///Common FIFO Access Data Field 15 Register %s Channel 0
pub mod cfdcfdf15_0;
/**CFDCFID_1 (rw) register accessor: Common FIFO Access ID Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfid_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfid_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfid_1`] module*/
#[doc(alias = "CFDCFID_1")]
pub type Cfdcfid1 = crate::Reg<cfdcfid_1::Cfdcfid1Spec>;
///Common FIFO Access ID Register %s Channel 1
pub mod cfdcfid_1;
/**CFDCFPTR_1 (rw) register accessor: Common FIFO Access Pointer Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfptr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfptr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfptr_1`] module*/
#[doc(alias = "CFDCFPTR_1")]
pub type Cfdcfptr1 = crate::Reg<cfdcfptr_1::Cfdcfptr1Spec>;
///Common FIFO Access Pointer Register %s Channel 1
pub mod cfdcfptr_1;
/**CFDCFFDCSTS_1 (rw) register accessor: Common FIFO Access CAN-FD Control/Status Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcffdcsts_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcffdcsts_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcffdcsts_1`] module*/
#[doc(alias = "CFDCFFDCSTS_1")]
pub type Cfdcffdcsts1 = crate::Reg<cfdcffdcsts_1::Cfdcffdcsts1Spec>;
///Common FIFO Access CAN-FD Control/Status Register %s Channel 1
pub mod cfdcffdcsts_1;
/**CFDCFDF0_1 (rw) register accessor: Common FIFO Access Data Field 0 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf0_1`] module*/
#[doc(alias = "CFDCFDF0_1")]
pub type Cfdcfdf0_1 = crate::Reg<cfdcfdf0_1::Cfdcfdf0_1Spec>;
///Common FIFO Access Data Field 0 Register %s Channel 1
pub mod cfdcfdf0_1;
/**CFDCFDF1_1 (rw) register accessor: Common FIFO Access Data Field 1 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf1_1`] module*/
#[doc(alias = "CFDCFDF1_1")]
pub type Cfdcfdf1_1 = crate::Reg<cfdcfdf1_1::Cfdcfdf1_1Spec>;
///Common FIFO Access Data Field 1 Register %s Channel 1
pub mod cfdcfdf1_1;
/**CFDCFDF2_1 (rw) register accessor: Common FIFO Access Data Field 2 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf2_1`] module*/
#[doc(alias = "CFDCFDF2_1")]
pub type Cfdcfdf2_1 = crate::Reg<cfdcfdf2_1::Cfdcfdf2_1Spec>;
///Common FIFO Access Data Field 2 Register %s Channel 1
pub mod cfdcfdf2_1;
/**CFDCFDF3_1 (rw) register accessor: Common FIFO Access Data Field 3 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf3_1`] module*/
#[doc(alias = "CFDCFDF3_1")]
pub type Cfdcfdf3_1 = crate::Reg<cfdcfdf3_1::Cfdcfdf3_1Spec>;
///Common FIFO Access Data Field 3 Register %s Channel 1
pub mod cfdcfdf3_1;
/**CFDCFDF4_1 (rw) register accessor: Common FIFO Access Data Field 4 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf4_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf4_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf4_1`] module*/
#[doc(alias = "CFDCFDF4_1")]
pub type Cfdcfdf4_1 = crate::Reg<cfdcfdf4_1::Cfdcfdf4_1Spec>;
///Common FIFO Access Data Field 4 Register %s Channel 1
pub mod cfdcfdf4_1;
/**CFDCFDF5_1 (rw) register accessor: Common FIFO Access Data Field 5 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf5_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf5_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf5_1`] module*/
#[doc(alias = "CFDCFDF5_1")]
pub type Cfdcfdf5_1 = crate::Reg<cfdcfdf5_1::Cfdcfdf5_1Spec>;
///Common FIFO Access Data Field 5 Register %s Channel 1
pub mod cfdcfdf5_1;
/**CFDCFDF6_1 (rw) register accessor: Common FIFO Access Data Field 6 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf6_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf6_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf6_1`] module*/
#[doc(alias = "CFDCFDF6_1")]
pub type Cfdcfdf6_1 = crate::Reg<cfdcfdf6_1::Cfdcfdf6_1Spec>;
///Common FIFO Access Data Field 6 Register %s Channel 1
pub mod cfdcfdf6_1;
/**CFDCFDF7_1 (rw) register accessor: Common FIFO Access Data Field 7 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf7_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf7_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf7_1`] module*/
#[doc(alias = "CFDCFDF7_1")]
pub type Cfdcfdf7_1 = crate::Reg<cfdcfdf7_1::Cfdcfdf7_1Spec>;
///Common FIFO Access Data Field 7 Register %s Channel 1
pub mod cfdcfdf7_1;
/**CFDCFDF8_1 (rw) register accessor: Common FIFO Access Data Field 8 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf8_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf8_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf8_1`] module*/
#[doc(alias = "CFDCFDF8_1")]
pub type Cfdcfdf8_1 = crate::Reg<cfdcfdf8_1::Cfdcfdf8_1Spec>;
///Common FIFO Access Data Field 8 Register %s Channel 1
pub mod cfdcfdf8_1;
/**CFDCFDF9_1 (rw) register accessor: Common FIFO Access Data Field 9 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf9_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf9_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf9_1`] module*/
#[doc(alias = "CFDCFDF9_1")]
pub type Cfdcfdf9_1 = crate::Reg<cfdcfdf9_1::Cfdcfdf9_1Spec>;
///Common FIFO Access Data Field 9 Register %s Channel 1
pub mod cfdcfdf9_1;
/**CFDCFDF10_1 (rw) register accessor: Common FIFO Access Data Field 10 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf10_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf10_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf10_1`] module*/
#[doc(alias = "CFDCFDF10_1")]
pub type Cfdcfdf10_1 = crate::Reg<cfdcfdf10_1::Cfdcfdf10_1Spec>;
///Common FIFO Access Data Field 10 Register %s Channel 1
pub mod cfdcfdf10_1;
/**CFDCFDF11_1 (rw) register accessor: Common FIFO Access Data Field 11 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf11_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf11_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf11_1`] module*/
#[doc(alias = "CFDCFDF11_1")]
pub type Cfdcfdf11_1 = crate::Reg<cfdcfdf11_1::Cfdcfdf11_1Spec>;
///Common FIFO Access Data Field 11 Register %s Channel 1
pub mod cfdcfdf11_1;
/**CFDCFDF12_1 (rw) register accessor: Common FIFO Access Data Field 12 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf12_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf12_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf12_1`] module*/
#[doc(alias = "CFDCFDF12_1")]
pub type Cfdcfdf12_1 = crate::Reg<cfdcfdf12_1::Cfdcfdf12_1Spec>;
///Common FIFO Access Data Field 12 Register %s Channel 1
pub mod cfdcfdf12_1;
/**CFDCFDF13_1 (rw) register accessor: Common FIFO Access Data Field 13 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf13_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf13_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf13_1`] module*/
#[doc(alias = "CFDCFDF13_1")]
pub type Cfdcfdf13_1 = crate::Reg<cfdcfdf13_1::Cfdcfdf13_1Spec>;
///Common FIFO Access Data Field 13 Register %s Channel 1
pub mod cfdcfdf13_1;
/**CFDCFDF14_1 (rw) register accessor: Common FIFO Access Data Field 14 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf14_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf14_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf14_1`] module*/
#[doc(alias = "CFDCFDF14_1")]
pub type Cfdcfdf14_1 = crate::Reg<cfdcfdf14_1::Cfdcfdf14_1Spec>;
///Common FIFO Access Data Field 14 Register %s Channel 1
pub mod cfdcfdf14_1;
/**CFDCFDF15_1 (rw) register accessor: Common FIFO Access Data Field 15 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf15_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf15_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdcfdf15_1`] module*/
#[doc(alias = "CFDCFDF15_1")]
pub type Cfdcfdf15_1 = crate::Reg<cfdcfdf15_1::Cfdcfdf15_1Spec>;
///Common FIFO Access Data Field 15 Register %s Channel 1
pub mod cfdcfdf15_1;
/**CFDTHLACC0 (r) register accessor: TX History List Access Registers 0

You can [`read`](crate::Reg::read) this register and get [`cfdthlacc0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdthlacc0`] module*/
#[doc(alias = "CFDTHLACC0")]
pub type Cfdthlacc0 = crate::Reg<cfdthlacc0::Cfdthlacc0Spec>;
///TX History List Access Registers 0
pub mod cfdthlacc0;
/**CFDTHLACC1 (rw) register accessor: TX History List Access Registers 1

You can [`read`](crate::Reg::read) this register and get [`cfdthlacc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdthlacc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdthlacc1`] module*/
#[doc(alias = "CFDTHLACC1")]
pub type Cfdthlacc1 = crate::Reg<cfdthlacc1::Cfdthlacc1Spec>;
///TX History List Access Registers 1
pub mod cfdthlacc1;
/**CFDRPGACC (rw) register accessor: RAM Test Page Access Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdrpgacc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrpgacc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdrpgacc`] module*/
#[doc(alias = "CFDRPGACC")]
pub type Cfdrpgacc = crate::Reg<cfdrpgacc::CfdrpgaccSpec>;
///RAM Test Page Access Registers %s
pub mod cfdrpgacc;
/**CFDTMID_0 (rw) register accessor: TX Message Buffer ID Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmid_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmid_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmid_0`] module*/
#[doc(alias = "CFDTMID_0")]
pub type Cfdtmid0 = crate::Reg<cfdtmid_0::Cfdtmid0Spec>;
///TX Message Buffer ID Register %s Channel 0
pub mod cfdtmid_0;
/**CFDTMPTR_0 (rw) register accessor: TX Message Buffer Pointer Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmptr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmptr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmptr_0`] module*/
#[doc(alias = "CFDTMPTR_0")]
pub type Cfdtmptr0 = crate::Reg<cfdtmptr_0::Cfdtmptr0Spec>;
///TX Message Buffer Pointer Register %s Channel 0
pub mod cfdtmptr_0;
/**CFDTMFDCTR_0 (rw) register accessor: TX Message Buffer CANFD Control Register %s Channel i

You can [`read`](crate::Reg::read) this register and get [`cfdtmfdctr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmfdctr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmfdctr_0`] module*/
#[doc(alias = "CFDTMFDCTR_0")]
pub type Cfdtmfdctr0 = crate::Reg<cfdtmfdctr_0::Cfdtmfdctr0Spec>;
///TX Message Buffer CANFD Control Register %s Channel i
pub mod cfdtmfdctr_0;
/**CFDTMDF0__0 (rw) register accessor: TX Message Buffer Data Field 0 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf0__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf0__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf0__0`] module*/
#[doc(alias = "CFDTMDF0__0")]
pub type Cfdtmdf0_0 = crate::Reg<cfdtmdf0__0::Cfdtmdf0_0Spec>;
///TX Message Buffer Data Field 0 Register %s Channel 0
pub mod cfdtmdf0__0;
/**CFDTMDF1__0 (rw) register accessor: TX Message Buffer Data Field 1 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf1__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf1__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf1__0`] module*/
#[doc(alias = "CFDTMDF1__0")]
pub type Cfdtmdf1_0 = crate::Reg<cfdtmdf1__0::Cfdtmdf1_0Spec>;
///TX Message Buffer Data Field 1 Register %s Channel 0
pub mod cfdtmdf1__0;
/**CFDTMDF2__0 (rw) register accessor: TX Message Buffer Data Field 2 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf2__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf2__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf2__0`] module*/
#[doc(alias = "CFDTMDF2__0")]
pub type Cfdtmdf2_0 = crate::Reg<cfdtmdf2__0::Cfdtmdf2_0Spec>;
///TX Message Buffer Data Field 2 Register %s Channel 0
pub mod cfdtmdf2__0;
/**CFDTMDF3__0 (rw) register accessor: TX Message Buffer Data Field 3 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf3__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf3__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf3__0`] module*/
#[doc(alias = "CFDTMDF3__0")]
pub type Cfdtmdf3_0 = crate::Reg<cfdtmdf3__0::Cfdtmdf3_0Spec>;
///TX Message Buffer Data Field 3 Register %s Channel 0
pub mod cfdtmdf3__0;
/**CFDTMDF4__0 (rw) register accessor: TX Message Buffer Data Field 4 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf4__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf4__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf4__0`] module*/
#[doc(alias = "CFDTMDF4__0")]
pub type Cfdtmdf4_0 = crate::Reg<cfdtmdf4__0::Cfdtmdf4_0Spec>;
///TX Message Buffer Data Field 4 Register %s Channel 0
pub mod cfdtmdf4__0;
/**CFDTMDF5__0 (rw) register accessor: TX Message Buffer Data Field 5 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf5__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf5__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf5__0`] module*/
#[doc(alias = "CFDTMDF5__0")]
pub type Cfdtmdf5_0 = crate::Reg<cfdtmdf5__0::Cfdtmdf5_0Spec>;
///TX Message Buffer Data Field 5 Register %s Channel 0
pub mod cfdtmdf5__0;
/**CFDTMDF6__0 (rw) register accessor: TX Message Buffer Data Field 6 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf6__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf6__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf6__0`] module*/
#[doc(alias = "CFDTMDF6__0")]
pub type Cfdtmdf6_0 = crate::Reg<cfdtmdf6__0::Cfdtmdf6_0Spec>;
///TX Message Buffer Data Field 6 Register %s Channel 0
pub mod cfdtmdf6__0;
/**CFDTMDF7__0 (rw) register accessor: TX Message Buffer Data Field 7 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf7__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf7__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf7__0`] module*/
#[doc(alias = "CFDTMDF7__0")]
pub type Cfdtmdf7_0 = crate::Reg<cfdtmdf7__0::Cfdtmdf7_0Spec>;
///TX Message Buffer Data Field 7 Register %s Channel 0
pub mod cfdtmdf7__0;
/**CFDTMDF8__0 (rw) register accessor: TX Message Buffer Data Field 8 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf8__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf8__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf8__0`] module*/
#[doc(alias = "CFDTMDF8__0")]
pub type Cfdtmdf8_0 = crate::Reg<cfdtmdf8__0::Cfdtmdf8_0Spec>;
///TX Message Buffer Data Field 8 Register %s Channel 0
pub mod cfdtmdf8__0;
/**CFDTMDF9__0 (rw) register accessor: TX Message Buffer Data Field 9 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf9__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf9__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf9__0`] module*/
#[doc(alias = "CFDTMDF9__0")]
pub type Cfdtmdf9_0 = crate::Reg<cfdtmdf9__0::Cfdtmdf9_0Spec>;
///TX Message Buffer Data Field 9 Register %s Channel 0
pub mod cfdtmdf9__0;
/**CFDTMDF10__0 (rw) register accessor: TX Message Buffer Data Field 10 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf10__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf10__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf10__0`] module*/
#[doc(alias = "CFDTMDF10__0")]
pub type Cfdtmdf10_0 = crate::Reg<cfdtmdf10__0::Cfdtmdf10_0Spec>;
///TX Message Buffer Data Field 10 Register %s Channel 0
pub mod cfdtmdf10__0;
/**CFDTMDF11__0 (rw) register accessor: TX Message Buffer Data Field 11 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf11__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf11__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf11__0`] module*/
#[doc(alias = "CFDTMDF11__0")]
pub type Cfdtmdf11_0 = crate::Reg<cfdtmdf11__0::Cfdtmdf11_0Spec>;
///TX Message Buffer Data Field 11 Register %s Channel 0
pub mod cfdtmdf11__0;
/**CFDTMDF12__0 (rw) register accessor: TX Message Buffer Data Field 12 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf12__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf12__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf12__0`] module*/
#[doc(alias = "CFDTMDF12__0")]
pub type Cfdtmdf12_0 = crate::Reg<cfdtmdf12__0::Cfdtmdf12_0Spec>;
///TX Message Buffer Data Field 12 Register %s Channel 0
pub mod cfdtmdf12__0;
/**CFDTMDF13__0 (rw) register accessor: TX Message Buffer Data Field 13 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf13__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf13__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf13__0`] module*/
#[doc(alias = "CFDTMDF13__0")]
pub type Cfdtmdf13_0 = crate::Reg<cfdtmdf13__0::Cfdtmdf13_0Spec>;
///TX Message Buffer Data Field 13 Register %s Channel 0
pub mod cfdtmdf13__0;
/**CFDTMDF14__0 (rw) register accessor: TX Message Buffer Data Field 14 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf14__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf14__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf14__0`] module*/
#[doc(alias = "CFDTMDF14__0")]
pub type Cfdtmdf14_0 = crate::Reg<cfdtmdf14__0::Cfdtmdf14_0Spec>;
///TX Message Buffer Data Field 14 Register %s Channel 0
pub mod cfdtmdf14__0;
/**CFDTMDF15__0 (rw) register accessor: TX Message Buffer Data Field X Register 15 Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf15__0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf15__0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf15__0`] module*/
#[doc(alias = "CFDTMDF15__0")]
pub type Cfdtmdf15_0 = crate::Reg<cfdtmdf15__0::Cfdtmdf15_0Spec>;
///TX Message Buffer Data Field X Register 15 Channel 0
pub mod cfdtmdf15__0;
pub use Cfdtmid0 as Cfdtmid32_0;
pub use cfdtmid_0 as cfdtmid32_0;
pub use Cfdtmptr0 as Cfdtmptr32_0;
pub use cfdtmptr_0 as cfdtmptr32_0;
pub use Cfdtmfdctr0 as Cfdtmfdctr32_0;
pub use cfdtmfdctr_0 as cfdtmfdctr32_0;
pub use Cfdtmdf0_0 as Cfdtmdf0_32_0;
pub use cfdtmdf0__0 as cfdtmdf0_32_0;
pub use Cfdtmdf1_0 as Cfdtmdf1_32_0;
pub use cfdtmdf1__0 as cfdtmdf1_32_0;
pub use Cfdtmdf2_0 as Cfdtmdf2_32_0;
pub use cfdtmdf2__0 as cfdtmdf2_32_0;
pub use Cfdtmdf3_0 as Cfdtmdf3_32_0;
pub use cfdtmdf3__0 as cfdtmdf3_32_0;
pub use Cfdtmdf4_0 as Cfdtmdf4_32_0;
pub use cfdtmdf4__0 as cfdtmdf4_32_0;
pub use Cfdtmdf5_0 as Cfdtmdf5_32_0;
pub use cfdtmdf5__0 as cfdtmdf5_32_0;
pub use Cfdtmdf6_0 as Cfdtmdf6_32_0;
pub use cfdtmdf6__0 as cfdtmdf6_32_0;
pub use Cfdtmdf7_0 as Cfdtmdf7_32_0;
pub use cfdtmdf7__0 as cfdtmdf7_32_0;
pub use Cfdtmdf8_0 as Cfdtmdf8_32_0;
pub use cfdtmdf8__0 as cfdtmdf8_32_0;
pub use Cfdtmdf9_0 as Cfdtmdf9_32_0;
pub use cfdtmdf9__0 as cfdtmdf9_32_0;
pub use Cfdtmdf10_0 as Cfdtmdf10_32_0;
pub use cfdtmdf10__0 as cfdtmdf10_32_0;
pub use Cfdtmdf11_0 as Cfdtmdf11_32_0;
pub use cfdtmdf11__0 as cfdtmdf11_32_0;
pub use Cfdtmdf12_0 as Cfdtmdf12_32_0;
pub use cfdtmdf12__0 as cfdtmdf12_32_0;
pub use Cfdtmdf13_0 as Cfdtmdf13_32_0;
pub use cfdtmdf13__0 as cfdtmdf13_32_0;
pub use Cfdtmdf14_0 as Cfdtmdf14_32_0;
pub use cfdtmdf14__0 as cfdtmdf14_32_0;
pub use Cfdtmdf15_0 as Cfdtmdf15_32_0;
pub use cfdtmdf15__0 as cfdtmdf15_32_0;
/**CFDTMID_1 (rw) register accessor: TX Message Buffer ID Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmid_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmid_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmid_1`] module*/
#[doc(alias = "CFDTMID_1")]
pub type Cfdtmid1 = crate::Reg<cfdtmid_1::Cfdtmid1Spec>;
///TX Message Buffer ID Register %s Channel 1
pub mod cfdtmid_1;
/**CFDTMPTR_1 (rw) register accessor: TX Message Buffer Pointer Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmptr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmptr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmptr_1`] module*/
#[doc(alias = "CFDTMPTR_1")]
pub type Cfdtmptr1 = crate::Reg<cfdtmptr_1::Cfdtmptr1Spec>;
///TX Message Buffer Pointer Register %s Channel 1
pub mod cfdtmptr_1;
/**CFDTMFDCTR_1 (rw) register accessor: TX Message Buffer CANFD Control Register %s Channel i

You can [`read`](crate::Reg::read) this register and get [`cfdtmfdctr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmfdctr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmfdctr_1`] module*/
#[doc(alias = "CFDTMFDCTR_1")]
pub type Cfdtmfdctr1 = crate::Reg<cfdtmfdctr_1::Cfdtmfdctr1Spec>;
///TX Message Buffer CANFD Control Register %s Channel i
pub mod cfdtmfdctr_1;
/**CFDTMDF0__1 (rw) register accessor: TX Message Buffer Data Field 0 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf0__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf0__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf0__1`] module*/
#[doc(alias = "CFDTMDF0__1")]
pub type Cfdtmdf0_1 = crate::Reg<cfdtmdf0__1::Cfdtmdf0_1Spec>;
///TX Message Buffer Data Field 0 Register %s Channel 1
pub mod cfdtmdf0__1;
/**CFDTMDF1__1 (rw) register accessor: TX Message Buffer Data Field 1 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf1__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf1__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf1__1`] module*/
#[doc(alias = "CFDTMDF1__1")]
pub type Cfdtmdf1_1 = crate::Reg<cfdtmdf1__1::Cfdtmdf1_1Spec>;
///TX Message Buffer Data Field 1 Register %s Channel 1
pub mod cfdtmdf1__1;
/**CFDTMDF2__1 (rw) register accessor: TX Message Buffer Data Field 2 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf2__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf2__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf2__1`] module*/
#[doc(alias = "CFDTMDF2__1")]
pub type Cfdtmdf2_1 = crate::Reg<cfdtmdf2__1::Cfdtmdf2_1Spec>;
///TX Message Buffer Data Field 2 Register %s Channel 1
pub mod cfdtmdf2__1;
/**CFDTMDF3__1 (rw) register accessor: TX Message Buffer Data Field 3 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf3__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf3__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf3__1`] module*/
#[doc(alias = "CFDTMDF3__1")]
pub type Cfdtmdf3_1 = crate::Reg<cfdtmdf3__1::Cfdtmdf3_1Spec>;
///TX Message Buffer Data Field 3 Register %s Channel 1
pub mod cfdtmdf3__1;
/**CFDTMDF4__1 (rw) register accessor: TX Message Buffer Data Field 4 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf4__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf4__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf4__1`] module*/
#[doc(alias = "CFDTMDF4__1")]
pub type Cfdtmdf4_1 = crate::Reg<cfdtmdf4__1::Cfdtmdf4_1Spec>;
///TX Message Buffer Data Field 4 Register %s Channel 1
pub mod cfdtmdf4__1;
/**CFDTMDF5__1 (rw) register accessor: TX Message Buffer Data Field 5 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf5__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf5__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf5__1`] module*/
#[doc(alias = "CFDTMDF5__1")]
pub type Cfdtmdf5_1 = crate::Reg<cfdtmdf5__1::Cfdtmdf5_1Spec>;
///TX Message Buffer Data Field 5 Register %s Channel 1
pub mod cfdtmdf5__1;
/**CFDTMDF6__1 (rw) register accessor: TX Message Buffer Data Field 6 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf6__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf6__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf6__1`] module*/
#[doc(alias = "CFDTMDF6__1")]
pub type Cfdtmdf6_1 = crate::Reg<cfdtmdf6__1::Cfdtmdf6_1Spec>;
///TX Message Buffer Data Field 6 Register %s Channel 1
pub mod cfdtmdf6__1;
/**CFDTMDF7__1 (rw) register accessor: TX Message Buffer Data Field 7 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf7__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf7__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf7__1`] module*/
#[doc(alias = "CFDTMDF7__1")]
pub type Cfdtmdf7_1 = crate::Reg<cfdtmdf7__1::Cfdtmdf7_1Spec>;
///TX Message Buffer Data Field 7 Register %s Channel 1
pub mod cfdtmdf7__1;
/**CFDTMDF8__1 (rw) register accessor: TX Message Buffer Data Field 8 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf8__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf8__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf8__1`] module*/
#[doc(alias = "CFDTMDF8__1")]
pub type Cfdtmdf8_1 = crate::Reg<cfdtmdf8__1::Cfdtmdf8_1Spec>;
///TX Message Buffer Data Field 8 Register %s Channel 1
pub mod cfdtmdf8__1;
/**CFDTMDF9__1 (rw) register accessor: TX Message Buffer Data Field 9 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf9__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf9__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf9__1`] module*/
#[doc(alias = "CFDTMDF9__1")]
pub type Cfdtmdf9_1 = crate::Reg<cfdtmdf9__1::Cfdtmdf9_1Spec>;
///TX Message Buffer Data Field 9 Register %s Channel 1
pub mod cfdtmdf9__1;
/**CFDTMDF10__1 (rw) register accessor: TX Message Buffer Data Field 10 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf10__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf10__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf10__1`] module*/
#[doc(alias = "CFDTMDF10__1")]
pub type Cfdtmdf10_1 = crate::Reg<cfdtmdf10__1::Cfdtmdf10_1Spec>;
///TX Message Buffer Data Field 10 Register %s Channel 1
pub mod cfdtmdf10__1;
/**CFDTMDF11__1 (rw) register accessor: TX Message Buffer Data Field 11 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf11__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf11__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf11__1`] module*/
#[doc(alias = "CFDTMDF11__1")]
pub type Cfdtmdf11_1 = crate::Reg<cfdtmdf11__1::Cfdtmdf11_1Spec>;
///TX Message Buffer Data Field 11 Register %s Channel 1
pub mod cfdtmdf11__1;
/**CFDTMDF12__1 (rw) register accessor: TX Message Buffer Data Field 12 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf12__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf12__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf12__1`] module*/
#[doc(alias = "CFDTMDF12__1")]
pub type Cfdtmdf12_1 = crate::Reg<cfdtmdf12__1::Cfdtmdf12_1Spec>;
///TX Message Buffer Data Field 12 Register %s Channel 1
pub mod cfdtmdf12__1;
/**CFDTMDF13__1 (rw) register accessor: TX Message Buffer Data Field 13 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf13__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf13__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf13__1`] module*/
#[doc(alias = "CFDTMDF13__1")]
pub type Cfdtmdf13_1 = crate::Reg<cfdtmdf13__1::Cfdtmdf13_1Spec>;
///TX Message Buffer Data Field 13 Register %s Channel 1
pub mod cfdtmdf13__1;
/**CFDTMDF14__1 (rw) register accessor: TX Message Buffer Data Field 14 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf14__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf14__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf14__1`] module*/
#[doc(alias = "CFDTMDF14__1")]
pub type Cfdtmdf14_1 = crate::Reg<cfdtmdf14__1::Cfdtmdf14_1Spec>;
///TX Message Buffer Data Field 14 Register %s Channel 1
pub mod cfdtmdf14__1;
/**CFDTMDF15__1 (rw) register accessor: TX Message Buffer Data Field X Register 15 Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf15__1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf15__1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfdtmdf15__1`] module*/
#[doc(alias = "CFDTMDF15__1")]
pub type Cfdtmdf15_1 = crate::Reg<cfdtmdf15__1::Cfdtmdf15_1Spec>;
///TX Message Buffer Data Field X Register 15 Channel 1
pub mod cfdtmdf15__1;
pub use Cfdtmid1 as Cfdtmid32_1;
pub use cfdtmid_1 as cfdtmid32_1;
pub use Cfdtmptr1 as Cfdtmptr32_1;
pub use cfdtmptr_1 as cfdtmptr32_1;
pub use Cfdtmfdctr1 as Cfdtmfdctr32_1;
pub use cfdtmfdctr_1 as cfdtmfdctr32_1;
pub use Cfdtmdf0_1 as Cfdtmdf0_32_1;
pub use cfdtmdf0__1 as cfdtmdf0_32_1;
pub use Cfdtmdf1_1 as Cfdtmdf1_32_1;
pub use cfdtmdf1__1 as cfdtmdf1_32_1;
pub use Cfdtmdf2_1 as Cfdtmdf2_32_1;
pub use cfdtmdf2__1 as cfdtmdf2_32_1;
pub use Cfdtmdf3_1 as Cfdtmdf3_32_1;
pub use cfdtmdf3__1 as cfdtmdf3_32_1;
pub use Cfdtmdf4_1 as Cfdtmdf4_32_1;
pub use cfdtmdf4__1 as cfdtmdf4_32_1;
pub use Cfdtmdf5_1 as Cfdtmdf5_32_1;
pub use cfdtmdf5__1 as cfdtmdf5_32_1;
pub use Cfdtmdf6_1 as Cfdtmdf6_32_1;
pub use cfdtmdf6__1 as cfdtmdf6_32_1;
pub use Cfdtmdf7_1 as Cfdtmdf7_32_1;
pub use cfdtmdf7__1 as cfdtmdf7_32_1;
pub use Cfdtmdf8_1 as Cfdtmdf8_32_1;
pub use cfdtmdf8__1 as cfdtmdf8_32_1;
pub use Cfdtmdf9_1 as Cfdtmdf9_32_1;
pub use cfdtmdf9__1 as cfdtmdf9_32_1;
pub use Cfdtmdf10_1 as Cfdtmdf10_32_1;
pub use cfdtmdf10__1 as cfdtmdf10_32_1;
pub use Cfdtmdf11_1 as Cfdtmdf11_32_1;
pub use cfdtmdf11__1 as cfdtmdf11_32_1;
pub use Cfdtmdf12_1 as Cfdtmdf12_32_1;
pub use cfdtmdf12__1 as cfdtmdf12_32_1;
pub use Cfdtmdf13_1 as Cfdtmdf13_32_1;
pub use cfdtmdf13__1 as cfdtmdf13_32_1;
pub use Cfdtmdf14_1 as Cfdtmdf14_32_1;
pub use cfdtmdf14__1 as cfdtmdf14_32_1;
pub use Cfdtmdf15_1 as Cfdtmdf15_32_1;
pub use cfdtmdf15__1 as cfdtmdf15_32_1;
