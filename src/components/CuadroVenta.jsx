import { useState } from "react";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Productos from "./Productos"
import TablaProductos from "./TablaProductos";
async function buscarProducto(filtrado) {
    return await invoke("get_productos_filtrado", { filtro: '' + filtrado });
}
function CuadroVenta({ venta, conf, prodFoc, isProd, busqueda,focuseado }) {
    const [total, setTotal] = useState(venta.monto_total);
    const [foc, setFoc] = useState(prodFoc);
    const [focused, setFocused] = useState(focuseado);
    const [rend, setRend] = useState(<section id="cuadro-venta">
        <Productos productos={venta.productos} conf={conf} prodFoc={foc} isProd={isProd} />
        <section id="monto-total">TOTAL {total}</section>
    </section>);


    function dibujarProductos(prods, conf) {
        setRend(<TablaProductos productos={prods} conf={conf} focuseado={focused}/>)
    }
    useEffect(() => {setFoc(prodFoc)}, [prodFoc])
    useEffect(() => {setFocused(focuseado)},[focuseado])
    useEffect(() => {
        if (busqueda) {
            buscarProducto(busqueda).then(prods => dibujarProductos(prods, conf));
        } else {
            setRend(<section id="cuadro-venta">
                <Productos productos={venta.productos} conf={conf} prodFoc={foc} isProd={isProd} />
                <section id="monto-total">TOTAL {total}</section>
            </section>)
        }
    }, [busqueda,foc,focused])
    return (rend)
}

export default CuadroVenta;