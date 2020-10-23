<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Customers</name>
   <tag></tag>
   <elementGuidId>c1de6bcd-f69e-4225-a79e-f4d1992ff786</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/api/v1/customer</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>5479eaf3-c06e-487b-b33f-ec2dead7b2bf</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>6c440eb3-998d-47c8-b543-0a755ee96efa</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'data[0].customerId', &quot;10&quot;)
WS.verifyElementPropertyValue(response, 'data[0].name', &quot;aa&quot;)
WS.verifyElementPropertyValue(response, 'data[0].description', &quot;Employee&quot;)
WS.verifyElementPropertyValue(response, 'data[0].isDeleted', &quot;0&quot;)

WS.verifyElementPropertyValue(response, 'data[1].customerId', &quot;5&quot;)
WS.verifyElementPropertyValue(response, 'data[1].name', &quot;alodia&quot;)
WS.verifyElementPropertyValue(response, 'data[1].description', &quot;Employee&quot;)
WS.verifyElementPropertyValue(response, 'data[1].isDeleted', &quot;0&quot;)

//WS.verifyElementPropertyValue(response, 'data[6].customerId', &quot;1&quot;)
//WS.verifyElementPropertyValue(response, 'data[6].name', &quot;Sasuke Uchiha&quot;)
//WS.verifyElementPropertyValue(response, 'data[6].description', &quot;&quot;)
//WS.verifyElementPropertyValue(response, 'data[6].isDeleted', &quot;0&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
